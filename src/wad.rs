use std::{
    path::Path,
    fs::File,
    io::Read,
    collections::LinkedList,
    mem::size_of, str::FromStr
};

use regex::Regex;
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;

use crate::{
    error::WadError,
    lump::{LumpInfo, LumpKind},
    models::{lump::Lump, operation::WadOperation},
    lumps::{
        palette::Palettes,
        unknown::Unknown, doom_image::DoomImage, flat::Flat
    }
};

/// Default re_name used by the `Wad` struct
pub const DEFAULT_RE_NAME: &'static str = r".*";

/// WAD types
#[derive(Debug, PartialEq)]
pub enum WadKind {
    Iwad,
    Pwad,
    Unknown
}

impl From<&[u8]> for WadKind {
    fn from(magic: &[u8]) -> Self {
        match magic {
            [0x49, 0x57, 0x41, 0x44] => Self::Iwad,
            [0x50, 0x57, 0x41, 0x44] => Self::Pwad,
            _ => Self::Unknown
        }
    }
}

/// Operations kind
#[derive(Debug, Clone, Copy)]
pub enum WadOperationKind {
    Dump,
    Save,
    SaveAs
}

impl Default for WadOperationKind{
    fn default() -> Self {
        Self::Dump
    }
}

impl FromStr for WadOperationKind {
    type Err = WadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "dump" => Self::Dump,
            "save" => Self::Save,
            "save_as" => Self::SaveAs,
            _ => return Err(WadError::InvalidOperation)
        };

        Ok(value)
    } 
}

/// Wad metadata (12 bytes)
pub struct WadInfo {
    /// Represents the file type (4 bytes)
    pub kind: WadKind,
    /// Holding the lumps amount (4 bytes) (little-endian)
    pub num_lumps: i32,
    /// Holding pointer to the directory location (4 bytes) (little-endian)
    pub dir_pos: i32
}

impl Default for WadInfo {
    fn default() -> Self {
        Self {
            kind: WadKind::Unknown,
            num_lumps: 0,
            dir_pos: 0,
        }
    }
}

impl From<&[u8]> for WadInfo {
    fn from(bytes: &[u8]) -> Self {
        Self {
            kind: WadKind::from(&bytes[0..4]),
            num_lumps: i32::from_le_bytes(
                bytes[4..8]
                    .try_into()
                    .unwrap_or_default()
            ),
            dir_pos: i32::from_le_bytes(
                bytes[8..12]
                    .try_into()
                    .unwrap_or_default()
            ),
        }
    }
}

lazy_static! {
    /// Flat start lump name
    static ref RE_F_START: Regex = Regex::new("F[0-9]+_START").unwrap();
    /// Flat end lump name
    static ref RE_F_END: Regex = Regex::new("F[0-9]+_END").unwrap();
    /// Patch/Sprite start lump name
    static ref RE_S_START: Regex = Regex::new("S[0-9]+_START").unwrap();
    /// Patch/Sprite end lump name
    static ref RE_S_END: Regex = Regex::new("S[0-9]+_END").unwrap();
}

/// WAD controller, it includes features like extract, dump, etc..
pub struct Wad {
    /// File type (IWAD or PWAD)
    info: WadInfo,
    /// Lumps <Name, Infos>
    lumps: LinkedHashMap<String, Box<dyn Lump>>,
    /// Buffer a.k.a file content
    buffer: Vec<u8>,
    /// Filter (regex)
    re_name: Regex,
    /// Palette
    pal: Palettes
}

impl Wad {
    pub fn new(re_name: Regex) -> Self {
        Self {
            info: WadInfo::default(),
            lumps: LinkedHashMap::new(),
            buffer: Vec::new(),
            re_name,
            pal: Palettes::default()
        }
    }

    /// Set the palette index
    pub fn set_palette(&mut self, value: usize) {
        self.pal.set_n(value);
    }

    /// Update the marker, handling the 0 bytes lumps like flat/patch delimiters
    fn set_marker(&self, marker: &mut LinkedList<LumpKind>, name: &str) {
        if RE_F_START.is_match(name) {
            marker.push_back(LumpKind::Flat);
        } 
        
        if RE_S_START.is_match(name) {
            marker.push_back(LumpKind::Patch);
        } 
        
        if RE_F_END.is_match(name) || RE_S_END.is_match(name){
            marker.pop_back();
        }
    }

    /// Iterating over the directory and filling `self.lumps`
    fn parse_dir(&mut self) {
        let size = size_of::<LumpInfo>();
        let mut marker: LinkedList<LumpKind> = LinkedList::new();

        for lump_num in 0..(self.info.num_lumps as usize) {
            let index = (self.info.dir_pos as usize) + (lump_num * size);
            let buffer = &self.buffer[index..index + size];

            // Get lump informations
            let info = LumpInfo::from(buffer);

            // Get the right lump
            let name = info.name_ascii();

            let mut lump: Box<dyn Lump> = match &*name {
                "PLAYPAL" => {
                    self.pal.set_info(info);
                    // Special case that must be parsed before copied
                    self.pal.parse(&self.buffer[(info.pos as usize)..]);
                    
                    Box::new(self.pal.clone())
                },

                "F_START" => {
                    marker.push_back(LumpKind::Flat);

                    Box::new(Unknown { info })
                },

                "F_END" => {
                    marker.pop_back();

                    Box::new(Unknown { info })
                },

                "S_START" | "SS_START" => {
                    marker.push_back(LumpKind::Patch);

                    Box::new(Unknown { info })
                },

                "S_END" | "SS_END" => {
                    marker.pop_back();

                    Box::new(Unknown { info })
                },

                _ => {
                    // Check the lump name with regex for marker
                    self.set_marker(&mut marker, &name);

                    if info.size > 0 && marker.len() > 0 {
                        let last = marker.back().unwrap();

                        match last {
                            LumpKind::Patch => {
                                // Match the engine (DOOM, HEXEN, etc...)
                                // Only intended for the DOOM engine right now
                                Box::new(DoomImage::new(
                                    info,
                                    self.pal.clone(),
                                ))
                            },
                            LumpKind::Flat => {
                                Box::new(Flat::new(
                                    info,
                                    self.pal.clone()
                                ))
                            },
                            _ => Box::new(Unknown { info })
                        }
                    } else {
                        Box::new(Unknown { info })
                    }
                }
            };

            // Fetch and decode data from the WAD buffer
            lump.parse(&self.buffer[(info.pos as usize)..]);

            // Add the lump to the hashmap
            self.lumps.insert(name, lump);
        }
    }

    /// Parse a buffer into lumps entries
    pub fn load<T: Into<Vec<u8>>>(
        &mut self,
        buffer: T
    ) -> Result<(), WadError> {
        self.buffer = buffer.into();

        // WAD informations
        if self.buffer.len() < 12 {
            return Err(WadError::Load(
                "The file size is too small."
            ))
        }

        // Check if the WAD is valid
        self.info = WadInfo::from(&self.buffer[0..12]);

        if self.info.kind == WadKind::Unknown {
            return Err(WadError::Type(
                "The file is not a WAD file."
            ))
        }

        // Parse lumps
        self.parse_dir();

        Ok(())
    }

    /// Load file content from a path
    pub fn load_from_file<P: AsRef<Path>>(
        &mut self, path: P
    ) -> Result<(), WadError> {
        let f = File::open(path);
        
        match f {
            Ok(mut file) => {
                let mut data = Vec::<u8>::new();

                match file.read_to_end(data.as_mut()) {
                    Ok(_) => {
                        self.load(data)
                    }
                    Err(e) => Err(WadError::Read(e.to_string()))
                }
            },
            Err(e) => Err(WadError::Read(e.to_string()))
        }
    }

    /// Iterate then calling a function on the matching lumps
    fn it_lumps(&self, f: fn(&Box<dyn Lump>) ) {
        for (name, lump) in self.lumps.iter() {
            if self.re_name.is_match(name) {
                f(lump);
            }
        }
    }
}

impl WadOperation for Wad {
    fn dump(&self) {
        self.it_lumps(| lump | println!("{}", lump));
    }

    fn save(&self) {
        self.it_lumps(| lump | lump.save());
    }

    fn save_as<P: AsRef<Path>>(&self, dir: P) {
        let dir = dir.as_ref().to_str().unwrap();

        for (name, lump) in self.lumps.iter() {
            if self.re_name.is_match(name) {
                lump.save_as(dir);
            }
        }
    }
}
