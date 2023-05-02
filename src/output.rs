use std::collections::HashMap;

use regex::Regex;

use crate::{
    wad::WadInfo,
    dir::LumpsDirectory,
    lump::LumpState,
};

/// str representing a regex matching a virtual lump (markers)
pub const VIRTUAL_LUMP_RE: &str = r"^([FPS]{1,2}_(?:START|END))|([FPS]{1}_?\d{0,2}_(?:START|END))$";

/// Manage the build of a new WAD file
pub struct WadOutput<'a> {
    /// The WAD controller
    info: WadInfo,
    /// Lumps offsets
    /// 
    /// Used because some lumps can have the same position (references)
    /// or virtual lumps
    offsets: HashMap<i32, i32>,
    dir: &'a LumpsDirectory,
    /// Destination buffer
    /// 
    /// It represents the final raw WAD file
    dest: Vec<u8>
}

impl<'a> WadOutput<'a> {
    pub fn new(
        mut info: WadInfo,
        dir: &'a LumpsDirectory,
    ) -> WadOutput<'a> {
        // Skip the WAD metadata size
        info.dir_pos = 12;

        Self {
            info,
            offsets: HashMap::new(),
            dir,
            dest: Vec::new(),
        }
    }

    /// Computes every lump offset
    /// 
    /// Used because more than one lump can have the same offset
    fn build_offsets(&mut self) {
        let mut offset = self.info.dir_pos + 16 * self.info.num_lumps;
        
        for (_, lump) in self.dir.lumps.iter() {
            let metadata = lump.data().metadata;
            
            if metadata.state == LumpState::Deleted {
                continue
            }

            if self.offsets.get(&metadata.pos).is_none() {
                self.offsets.insert(
                    metadata.pos,
                    offset
                );
            }

            offset += metadata.size;
        }
    }

    /// Write the file entries into `self.dest`
    fn build_file_entries(&mut self) {
        let re = Regex::new(VIRTUAL_LUMP_RE).unwrap();

        for (_, lump) in self.dir.lumps.iter() {
            let mut metadata = lump.data().metadata;
            
            if metadata.state == LumpState::Deleted {
                continue
            }
            
            let offset = self.offsets.get(&metadata.pos).unwrap();
            if re.is_match(&metadata.name_ascii()) {
                metadata.pos = 0;
            } else {
                metadata.pos = *offset;
            }

            // Write the lump metadata
            self.dest.append(&mut metadata.into());
        }
    }

    /// Write the lumps content into `self.dest`
    /// 
    /// Running 2 loops instead of one has better performance, 
    /// because one loop implies to use `self.dest.insert` and it copies
    /// values instead of using references
    fn build_raw_lumps(&mut self) {
        for (_, lump) in self.dir.lumps.iter() {
            if lump.data().metadata.state != LumpState::Deleted {
                // Write the raw lump content
                self.dest.append(&mut lump.data().buffer);
            }
        }
    }

    /// Build the new WAD file into `self.dest`
    /// 
    /// It requires a source WAD abstraction
    /// aka `self.dir` and the metadatas aka `self.info`
    pub fn build(&mut self) {
        // Write the WAD metadata
        self.dest.append(&mut self.info.into());

        self.build_offsets();
        self.build_file_entries();
        self.build_raw_lumps();
    }

    /// Returns the WAD output
    pub fn buffer(&self) -> Vec<u8> {
        self.dest.clone()
    }
}
