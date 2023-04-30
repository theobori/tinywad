use crate::{
    wad::WadInfo,
    dir::LumpsDirectory,
    lump::LumpState,
    header::{
        Header,
        HeaderKind
    },
    models::lump::Lump
};

/// Manage the build of a new WAD file
pub struct WadOutput<'a> {
    /// The WAD controller
    info: WadInfo,
    /// Offsets that will be overwriting on the raw buffer
    offsets: Vec<i32>,
    dir: &'a LumpsDirectory,
    /// Destination buffer
    /// 
    /// It represents the final raw WAD file
    dest: Vec<u8>
}

impl<'a> WadOutput<'a> {
    pub fn new(
        info: WadInfo,
        dir: &'a LumpsDirectory,
        src: &Vec<u8>
    ) -> WadOutput<'a> {
        Self {
            info,
            offsets: vec![0; info.num_lumps as usize],
            dir,
            dest: src.clone(),
        }
    }

    /// Overwritting the 8 first bytes of the lump `lump`
    fn write_high_byte(
        &mut self,
        lump: &Box<dyn Lump>,
        i: usize
    ) {
        let mut metadata = lump.data().metadata;
        let dir_pos = self.info.dir_pos as usize;

        if metadata.is_overwritable() == false {
            return
        }

        metadata.pos += self.offsets[i];

        let mut bytes = i32::to_le_bytes(metadata.pos).to_vec();
        bytes.append(&mut i32::to_le_bytes(metadata.size).to_vec());

        for (j, byte) in bytes.iter().enumerate() {
            self.dest[dir_pos + (i * 16) + j] = *byte;
        }
    }

    /// It will overwrite the 8 first bytes of `LumpInfo`
    /// of every lump in `self.dest`
    fn write_high_bytes(&mut self) {
        let keys: Vec<&String> = self.dir.lumps.keys().collect();

        for i in 0..self.offsets.len() {
            let key = keys[i];
            let lump = self.dir.lumps
                .get(key)
                .unwrap();
            
            self.write_high_byte(lump, i)
        }
    }

    /// Delete the padding chunks
    fn delete_chunks(&mut self) {
        let mut i = 0;
        let header_len = HeaderKind::Delete.to_string().len() + 4;

        while i < self.dest.len() {
            if i + header_len > self.dest.len() {
                break
            }
    
            let chunk = &self.dest[i..i + header_len];
            let header = Header::from(chunk);

            if header.kind == HeaderKind::Delete {
                self.dest.drain(i..i + header.size() as usize);
                continue
            }
            
            i += 1;
        }
    }

    /// This method is going to parse `self.dest` then process its data
    /// 
    /// It will operates with the headers written on `self.dest`
    /// and writes some metadata (size, lumps directory position, etc..)
    fn overwrite(&mut self, dir_pos: i32) {
        self.write_high_bytes();
        self.delete_chunks();

        // WAD Metadata
        let mut bytes = i32::to_le_bytes(
            self.dir.alive_count() as i32
        ).to_vec();
        bytes.append(
            &mut i32::to_le_bytes(
                dir_pos as i32
            ).to_vec()
        );
       
        for (i, byte) in bytes.iter().enumerate() {
            self.dest[4 + i] = *byte;
        }
    }

    /// Writes the whole header at the starting position `pos`
    fn write_header(&mut self, header: Header, pos: usize) {
        let bytes: Vec<u8> = header.into();
        let size = bytes.len();

        if pos + size >= self.dest.len() {
            return
        }

        for i in 0..size {
            self.dest[pos + i] = bytes[i]
        }
    }

    /// Build the new WAD file into `self.dest`
    /// 
    /// It requires a source WAD buffer and its abstraction
    /// aka `self.dir` and the metadatas as `self.info`
    pub fn build(&mut self) {
        let mut dir_pos = self.info.dir_pos;

        for (i, (_, lump)) in self.dir.lumps.iter().enumerate() {
            let data = lump.data();
            let file_entry = self.info.dir_pos as usize + 16 * i;
            
            match data.metadata.state {
                LumpState::Deleted => {
                    if self.info.dir_pos > data.metadata.pos {
                        dir_pos -= data.metadata.size;
                    }

                    self.write_header(
                        Header::new(
                            HeaderKind::Delete,
                            16
                        ),
                        file_entry
                    );

                    self.write_header(
                        Header::new(
                            HeaderKind::Delete,
                            data.metadata.size
                        ),
                        data.metadata.pos as usize
                    );

                    for offset_index in i..self.offsets.len() {
                        self.offsets[offset_index] -= data.metadata.size;
        
                        if data.metadata.pos > self.info.dir_pos {
                            self.offsets[offset_index] -= 16;
                        }
                    }

                }
                // TODO: Updated state to handle dynamic size (?)
                _ => {},
            }
        }

        self.overwrite(dir_pos);
    }

    /// Returns the WAD output
    pub fn buffer(&self) -> Vec<u8> {
        self.dest.clone()
    }
}
