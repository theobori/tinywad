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
            "Name: {} -- Size: {}",
            self.info.name(),
            self.info.size
        )
    }
}

impl Lump for Unknown {
    fn parse(&mut self, buffer: &[u8]) {

    }

    fn save_as(&self, path: &str) {
        
    }

    fn save(&self) {
        
    }
}
