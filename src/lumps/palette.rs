use std::fmt::{
    Display,
    Result
};

use crate::{
    models::lump::Lump,
    lump::LumpInfo
};

const PALETTE_SIZE: usize = 768;

/// PLAYPAL
#[derive(Clone)]
pub struct Palettes {
    /// Lump metadata
    pub info: LumpInfo,
    /// Data a.k.a the palettes (array of 768 bytes -> 256 * 3)
    pub palettes: Vec<Vec<u8>>
}

impl Palettes {
    /// Get the palettes
    pub fn palettes(&self) -> Vec<Vec<u8>> {
        self.palettes.clone()
    }

    /// Get a palette
    pub fn palette(&self, n: usize) -> Option<Vec<u8>> {
        self.palettes.get(n).cloned()
    }
}

impl Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let amount = self.info.size as usize / PALETTE_SIZE;

        write!(
            f,
            "Name: {} -- Size: {} -- Palettes amount: {}",
            self.info.name(),
            self.info.size,
            amount
        )
    }
}

impl Lump for Palettes {
    fn parse(&mut self, buffer: &[u8]) {
        // Reset the vector if the method is called multiple time by mistake
        self.palettes.clear();

        for i in (0..self.info.size as usize).step_by(PALETTE_SIZE) {
            let palette = &buffer[i..i + PALETTE_SIZE];
    
            self.palettes.push(palette.to_vec());
        }
    }

    fn save_as(&self, path: &str) {
        todo!()
    }

    fn save(&self) {
        todo!()
    }
}
