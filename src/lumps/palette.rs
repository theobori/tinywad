use std::{fmt::{
    Display,
    Result
}, path::Path};

use crate::{
    models::lump::Lump,
    lump::LumpData,
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
    /// Data a.k.a the palettes (array of 768 bytes -> 256 * 3)
    pub palettes: Vec<Palette>,
    /// Get the `n` palette
    n: usize,
    /// Raw file buffer
    data: LumpData
}

impl Default for Palettes {
    fn default() -> Self {
        Self {
            palettes: Vec::new(),
            n: 0,
            data: LumpData::default()
        }
    }
}

impl Palettes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the palettes
    pub fn palettes(&self) -> Vec<Palette> {
        self.palettes.clone()
    }

    /// Get a palette
    pub fn palette(&self) -> Option<Palette> {
        self.palettes.get(self.n).cloned()
    }

    /// Get a palette in a byte (u8) format
    pub fn palette_as_bytes(&self, n: usize) -> Vec<u8> {
        let palette = self.palettes.get(n).cloned();

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

    /// Set the lump data
    pub fn set_data(&mut self, data: LumpData) {
        self.data = data;
    }

    /// Set the `n` property
    pub fn set_n(&mut self, value: usize) {
        self.n = value;
    }
}

impl Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let amount = self.data.metadata.size as usize / PALETTE_SIZE;

        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Palettes amount: {}",
            self.data.metadata.name_ascii(),
            self.data.metadata.size,
            self.data.metadata.pos,
            amount
        )
    }
}

impl Lump for Palettes {
    fn parse(&mut self) {
        let buffer = &*self.data.buffer;
        // Reset the vector if the method is called multiple time by mistake
        self.palettes.clear();

        for i in (0..self.data.metadata.size as usize).step_by(PALETTE_SIZE) {
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

    fn save(&self, dir: &str) {
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

    fn data(&self) -> crate::lump::LumpData {
        self.data.clone()
    }

    fn set_data(&mut self, data: LumpData) {
        self.data = data;
    }

    fn update(&mut self, _buffer: &Vec<u8>) {
        todo!()
    }
}
