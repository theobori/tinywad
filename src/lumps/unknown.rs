use std::fmt::{
    Display,
    Result
};

use crate::{
    models::lump::Lump,
    lump::LumpData
};

/// Just an unknown or unimplemented lump
#[derive(Clone)]
pub struct Unknown {
    /// Lump data
    pub data: LumpData,
}

impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}",
            self.data.metadata.id_ascii(),
            self.data.metadata.size,
            self.data.metadata.pos
        )
    }
}

impl Lump for Unknown {
    fn parse(&mut self) {

    }

    fn save(&self, _dir: &str) {
    
    }

    fn data(&self) -> crate::lump::LumpData {
        self.data.clone()
    }

    fn set_data(&mut self, data: LumpData) {
        self.data = data;
    }

    fn update(&mut self, _buffer: &Vec<u8>) {
        
    }
}
