use std::{
    path::Path,
    fs::File,
    io::Read,
   str::FromStr
};

use linked_hash_map::LinkedHashMap;
use regex::Regex;

use crate::{
    error::WadError,
    models::operation::WadOperation,
    dir::LumpsDirectory, lumps::palette::Palettes
};

/// Default re_name used by the `Wad` struct
pub const DEFAULT_RE_NAME: &'static str = r".*";

/// WAD types
#[derive(Debug, PartialEq, Clone, Copy)]
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
#[derive(Clone, Copy)]
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

/// WAD controller, it includes features like extract, dump, etc..
pub struct Wad {
    /// File type (IWAD or PWAD)
    info: WadInfo,
    /// Buffer a.k.a file content
    buffer: Vec<u8>,
    /// Filter (regex)
    re_name: Regex,
    /// Lumps directory
    dir: LumpsDirectory
}

impl Wad {
    pub fn new(re_name: Regex) -> Self {
        Self {
            info: WadInfo::default(),
            buffer: Vec::new(),
            re_name,
            dir: LumpsDirectory {
                lumps: LinkedHashMap::new(),
                pal: Palettes::default()
            }
        }
    }

    /// Set a palette that will be applied on every lump
    pub fn set_palette(&mut self, value: usize) {
        self.dir.set_palette(value);
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
        self.dir.parse(self.info, &self.buffer);

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
}

impl WadOperation for Wad {
    fn dump(&self) {
        self.dir.callback_lumps(
            self.re_name.clone(),
            | lump | println!("{}", lump)
        );
    }

    fn save(&self) {
        self.dir.callback_lumps(
            self.re_name.clone(),
            | lump | lump.save()
        );
    }

    fn save_as<P: AsRef<Path>>(&self, dir: P) {
        let dir = dir.as_ref().to_str().unwrap();

        self.dir.callback_lumps(
            self.re_name.clone(),
            | lump | lump.save_as(dir)
        );
    }
}
