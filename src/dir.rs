use std::{
    collections::LinkedList,
    mem::size_of,
    cell::RefCell,
    rc::Rc
};
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use regex::Regex;

use crate::{
    models::lump::Lump,
    lump::{
        LumpKind,
        LumpInfo
    },
    lumps::{
        doom_image::DoomImage,
        flat::Flat,
        unknown::Unknown, palette::Palettes
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
}

const MAX_PAL: usize = 13;

/// Representing the lumps directory data
pub struct LumpsDirectory {
    /// Lumps hashmap <Name, Infos>
    pub lumps: LinkedHashMap<String, Box<dyn Lump>>,
    /// Palette
    pub pal: Palettes
}

impl LumpsDirectory {
    /// Iterate then calling a function on the matching lumps
    pub fn callback_lumps<F: Fn(&Box<dyn Lump>)>(
        &self,
        re: Regex,
        f: F
    ) {
        for (name, lump) in self.lumps.iter() {
            if re.is_match(name) {
                f(lump);
            }
        }
    }

    /// Remove matching index
    pub fn remove_lumps(&mut self, re: Regex) {
        for name in self.matches(re) {
            self.lumps.remove(&name);
        }
    }

    /// Get the matching lump names
    pub fn matches(&self, re: Regex) -> Vec<String> {
        let mut ret = Vec::new();

        for (name, _) in self.lumps.iter() {
            if re.is_match(name) {
                ret.push((*name).clone());
            }
        }
        
        ret
    }

    /// Set the palette index
    pub fn set_palette(&mut self, value: usize) {
        self.pal.set_n(value % MAX_PAL);
    }

    /// Update the marker, handling the 0 bytes lumps like flat/patch delimiters
    fn set_marker(
        &self,
        marker: &mut LinkedList<LumpKind>,
        name: &str
    ) {
        if RE_F_START.is_match(name) {
            marker.push_back(LumpKind::Flat);
        }
        
        if RE_S_START.is_match(name) {
            marker.push_back(LumpKind::Patch);
        } 
        
        if RE_F_END.is_match(name) {
            marker.pop_back();
        }

        if RE_S_END.is_match(name) {
            marker.pop_back();
        }

        // Add marker for music
    }

    /// Iterating over the directory and filling `self.lumps`
    pub fn parse(
        &mut self,
        info: WadInfo,
        buffer: Rc<RefCell<Vec<u8>>>
    ) {
        let size = size_of::<LumpInfo>();
        let mut marker: LinkedList<LumpKind> = LinkedList::new();

        // Link the palette buffer
        self.pal.set_raw(buffer.clone());

        for lump_num in 0..(info.num_lumps as usize) {
            let index = (info.dir_pos as usize) + (lump_num * size);

            // Get lump informations
            let info = LumpInfo::from(
                &buffer.borrow()[index..index + size]
            );
            // Get the right lump
            let name = info.name_ascii();

            let mut lump: Box<dyn Lump> = match &*name {
                "PLAYPAL" => {
                    self.pal.set_info(info);
                    // Special case that must be parsed before copied
                    self.pal.parse();
                    
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
                                    buffer.clone()
                                ))
                            },
                            LumpKind::Flat => {
                                Box::new(Flat::new(
                                    info,
                                    self.pal.clone(),
                                    buffer.clone()
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
            lump.parse();

            // Add the lump to the hashmap
            self.lumps.insert(name, lump);
        }
    }
}
