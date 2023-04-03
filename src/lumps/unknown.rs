use std::fmt::{
    Display,
    Result
};

use crate::{
    models::lump::Lump,
    lump::{LumpInfo, LumpData}
};

/// Just an unknown or unimplemented lump
pub struct Unknown {
    /// Lump data
    pub data: LumpData,
}

impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}",
            self.data.metadata.name_ascii(),
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
}
