use std::fmt::{
    Display,
    Result
};

use crate::{
    models::lump::Lump,
    lump::LumpInfo
};

/// Just an unknown or unimplemented lump
pub struct Unknown {
    /// Lump metadata
    pub info: LumpInfo,
}

impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}",
            self.info.name_ascii(),
            self.info.size,
            self.info.pos
        )
    }
}

impl Lump for Unknown {
    fn parse(&mut self, _buffer: &[u8]) {

    }

    fn save_as(&self, _dir: &str) {
    
    }
}
