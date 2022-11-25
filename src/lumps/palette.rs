use std::{fmt::{
    Display,
    Result
}, path::Path};

use crate::{
    models::lump::Lump,
    lump::LumpInfo,
    properties::color::ColorRgb
};

extern crate image;

/// Palette size in bytes
pub const PALETTE_SIZE: usize = 768;
/// Palette pixel size in bytes
pub const PIXEL_SIZE: usize = 3;

/// Palette
pub type Palette = Vec<ColorRgb>;

/// PLAYPAL
#[derive(Clone)]
pub struct Palettes {
    /// Lump metadata
    pub info: LumpInfo,
    /// Data a.k.a the palettes (array of 768 bytes -> 256 * 3)
    pub palettes: Vec<Palette>
}

impl Default for Palettes {
    fn default() -> Self {
        Self {
            info: LumpInfo::default(),
            palettes: Vec::new()
        }
    }
}

impl Palettes {
    /// Get the palettes
    pub fn palettes(&self) -> Vec<Palette> {
        self.palettes.clone()
    }

    /// Get a palette
    pub fn palette(&self, n: usize) -> Option<Palette> {
        self.palettes.get(n).cloned()
    }

    /// Get a palette in a byte (u8) format
    pub fn palette_as_bytes(&self, n: usize) -> Vec<u8> {
        let palette = self.palette(n);

        if palette.is_none() {
            return Vec::new();
        }
        
        let mut ret = Vec::new();

        for rgb in palette.unwrap() {
            let (r, g, b, a) = rgb.into();

            ret.push(r);
            ret.push(g);
            ret.push(b);
            ret.push(a);
        }

        ret            
    }
}

impl Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let amount = self.info.size as usize / PALETTE_SIZE;

        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Palettes amount: {}",
            self.info.name_ascii(),
            self.info.size,
            self.info.pos,
            amount
        )
    }
}

impl Lump for Palettes {
    fn parse(&mut self, buffer: &[u8]) {
        // Reset the vector if the method is called multiple time by mistake
        self.palettes.clear();

        for i in (0..self.info.size as usize).step_by(PALETTE_SIZE) {
            let mut palette = Vec::new();

            for pixel_pos in (0..PALETTE_SIZE).step_by(PIXEL_SIZE) {
                let pos = i + pixel_pos;
                let bytes = &buffer[pos..pos + PIXEL_SIZE];
                
                palette.push(
                    ColorRgb::from((
                        bytes[0],
                        bytes[1],
                        bytes[2]
                    )
                ));
            }
    
            self.palettes.push(palette);
        }
    }

    fn save_as(&self, dir: &str) {
        // Extract every palette as a single file
        for pal_index in 0..self.palettes.len() {
            let path = format!(
                "{}/PAL_{}.png",
                dir,
                pal_index
            );

            // Save the palette
            image::save_buffer(
                Path::new(&path),
                &self.palette_as_bytes(pal_index),
                16,
                16,
                image::ColorType::Rgba8
            ).unwrap();
            
        }
        
    }
}
