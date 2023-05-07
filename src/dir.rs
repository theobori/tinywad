use std::collections::{LinkedList, HashMap};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    models::lump::Lump,
    lump::{
        LumpKind,
        LumpInfo,
        LumpData, LumpState
    },
    lumps::{
        patch::DoomImage,
        flat::Flat,
        unknown::Unknown,
        palette::Palettes,
        music::lump::DoomMusic
    }, wad::WadInfo
};

lazy_static! {
    /// Flat start lump name
    static ref RE_F_START: Regex = Regex::new("F[0-9]+_START").unwrap();
    /// Flat end lump name
    static ref RE_F_END: Regex = Regex::new("F[0-9]+_END").unwrap();
    /// Patch/Sprite start lump name
    static ref RE_S_START: Regex = Regex::new("S[0-9]+_START").unwrap();
    /// Patch/Sprite end lump name
    static ref RE_S_END: Regex = Regex::new("S[0-9]+_END").unwrap();
    /// DOOM games lump name
    static ref RE_DOOM_MUSIC: Regex = Regex::new("^D_").unwrap();
}

/// DOOM Palette max value
pub const MAX_PAL: usize = 13;

/// Representing the lumps directory data
pub struct LumpsDirectory {
    /// Lumps hashmap <Name, Infos>
    pub lumps: Vec<Box<dyn Lump>>,
    /// Palette
    pub pal: Palettes,
    /// Used during the directory parsing
    marker: LinkedList<LumpKind>
}

impl Default for LumpsDirectory {
    fn default() -> Self {
        Self {
            lumps: Vec::new(),
            pal: Palettes::default(),
            marker: LinkedList::new() }
    }
}

impl LumpsDirectory {
    pub fn new() -> Self {
        Self::default()
    }
    /// Iterate then calling a function on the matching lumps
    pub fn callback_lumps<F: Fn(&Box<dyn Lump>)>(
        &self,
        re: Regex,
        f: F
    ) {
        for lump in self.lumps.iter() {
            let name = lump.data().metadata.name_ascii();
            
            if re.is_match(&name) {
                f(lump);
            }
        }
    }

    /// Iterate then calling a function on the matching mutable lumps
    pub fn callback_lumps_mut<F: Fn(&mut Box<dyn Lump>)>(
        &mut self,
        re: Regex,
        f: F
    ) {
        for lump in self.lumps.iter_mut() {
            let name = lump.data().metadata.name_ascii();
            
            if re.is_match(&name) {
                f(lump);
            }
        }
    }

    /// Remove matching index
    pub fn remove_lumps(&mut self, re: Regex) -> usize {
        let indexes = self.indexes(re);

        for index in indexes.iter() {
            let lump = self.lumps.get_mut(*index).unwrap();
            let mut data = lump.data();

            data.metadata.state = LumpState::Deleted;

            lump.set_data(data);
        }

        indexes.len()
    }

    /// Returns the indexes index
    pub fn indexes(&self, re: Regex) -> Vec<usize> {
        let mut ret = Vec::new();

        for (i, lump) in self.lumps.iter().enumerate() {
            let name = lump.data().metadata.id_ascii();

            if re.is_match(&name) {
                ret.push(i);
            }
        }
        
        ret
    }

    /// Returns a lump index by name
    pub fn index(&self, name: &str) -> Option<usize> {
        for (i, lump) in self.lumps.iter().enumerate() {
            if &*lump.data().metadata.id_ascii() == name {
                return Some(i)
            }
        }
        
        None
    }

    /// Returns a lump by name
    pub fn lump(&self, name: &str) -> Option<&Box<dyn Lump>> {
        let index = self.index(name);

        if index.is_none() {
            return None
        }

        Some(
            self.lumps
                .get(index.unwrap())
                .unwrap()
        )
    }

    /// Set the palette index
    pub fn set_palette(&mut self, value: usize) {
        self.pal.set_n(value % MAX_PAL);
    }

    /// Update the marker, handling the 0 bytes lumps like flat/patch delimiters
    fn set_marker(&mut self, name: &str) {
        if RE_F_START.is_match(name) {
            self.marker.push_back(LumpKind::Flat);
        }
        
        if RE_S_START.is_match(name) {
            self.marker.push_back(LumpKind::Patch);
        } 
        
        if RE_F_END.is_match(name) {
            self.marker.pop_back();
        }

        if RE_S_END.is_match(name) {
            self.marker.pop_back();
        }
    }

    fn pop_marker(
        &mut self,
        metadata: &LumpInfo,
        name: &str,
        mut data: LumpData
    ) -> Box<dyn Lump> {
        self.set_marker(name);

        if metadata.size <= 0 || self.marker.len() <= 0 {
            return Box::new(Unknown { data })
        }

        let last = self.marker.back().unwrap();

        match last {
            LumpKind::Patch => {
                data.kind = LumpKind::Patch;

                Box::new(DoomImage::new(
                    self.pal.clone(),
                    data
                ))
            },
            LumpKind::Flat => {
                data.kind = LumpKind::Flat;
                Box::new(Flat::new(
                    self.pal.clone(),
                    data
                ))
            },
            _ => Box::new(Unknown { data })
        }
    }

    /// Iterating over the directory and filling `self.lumps`
    pub fn parse(&mut self, info: WadInfo, buffer: &Vec<u8>) {
        self.lumps.clear();
        self.marker.clear();
    
        let mut marker: LinkedList<LumpKind> = LinkedList::new();
        // Preventing multiple names
        let mut names: HashMap<String, usize> = HashMap::new();

        for lump_num in 0..(info.num_lumps as usize) {
            let index = (info.dir_pos as usize) + (lump_num * 16);
            
            // Get lump informations then data
            let mut metadata = LumpInfo::from(
                &buffer[index..index + 16]
            );
            let pos = metadata.pos as usize;
            let size = metadata.size as usize;
            let name = metadata.name_ascii();

            let id = match names.get(&name) {
                Some(count) => {
                    let value = count + 1;
                    names.insert(name.clone(), value);

                    format!("{}{}", name, value)
                },
                None => {
                    names.insert(name.clone(), 0);
                    
                    name
                    .clone()
                }
            };

            for (i, byte) in id.as_bytes().iter().enumerate() {
                metadata.id[i] = *byte;
            }
            let mut data = LumpData {
                buffer: buffer[pos..pos + size].to_vec(),
                metadata,
                kind: LumpKind::Unknown,
            };
            
            let mut lump: Box<dyn Lump> = match &*name {
                "PLAYPAL" => {
                    data.kind = LumpKind::Palette;
                    
                    self.pal.set_data(data);
                    // Special case that must be parsed before copied
                    self.pal.parse();
                    
                    Box::new(self.pal.clone())
                },

                "F_START" => {
                    marker.push_back(LumpKind::Flat);

                    Box::new(Unknown { data })
                },

                "S_START" | "SS_START" => {
                    marker.push_back(LumpKind::Patch);

                    Box::new(Unknown { data })
                },

                "F_END" | "S_END" | "SS_END" => {
                    marker.pop_back();

                    Box::new(Unknown { data })
                },

                "TITLEPIC" => {
                    Box::new(DoomImage::new(
                        self.pal.clone(),
                        data
                    ))
                },

                _ => {
                    if RE_DOOM_MUSIC.is_match(&name) {
                        Box::new(DoomMusic::new(data))
                    } else {
                        self.pop_marker(&metadata, &name, data)
                    }
                }
            };

            // Fetch and decode data from the WAD buffer
            lump.parse();

            // Add the lump to the hashmap
            self.lumps.push(lump);
        }
    }
}
