use std::{
    fmt::{Display, Error},
    path::Path,
};

use crate::{error::WadError, lump::LumpData, lumps::palette::Palettes, models::lump::Lump};

extern crate image;

/// Flat width
pub const FLAT_W: usize = 64;
/// Flat height
pub const FLAT_H: usize = 64;
/// Flat size
pub const FLAT_SIZE: usize = FLAT_W * FLAT_H;

/// Represents a Flat
#[derive(Clone)]
pub struct Flat {
    /// Array used to store the DOOM image data before converting it into bitmap
    pixels: Vec<u8>,
    /// Attached palettes
    palettes: Palettes,
    /// Lump data
    data: LumpData,
}

impl Flat {
    pub fn new(palettes: Palettes, data: LumpData) -> Self {
        Self {
            pixels: Vec::new(),
            palettes,
            data,
        }
    }
}

impl Display for Flat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Width: {}, Height: {}",
            self.data.metadata.name_ascii(),
            self.data.metadata.size,
            self.data.metadata.pos,
            FLAT_W,
            FLAT_H
        )
    }
}

impl Lump for Flat {
    fn parse(&mut self) -> Result<(), WadError> {
        let buffer = &*self.data.buffer;
        let palette = match self.palettes.palette() {
            Some(value) => value,
            None => return Err(WadError::Parse(String::from("Invalid palette"))),
        };

        for i in 0..FLAT_SIZE {
            let byte = buffer[i];
            let (r, g, b, _) = palette[byte as usize].into();

            self.pixels.push(r);
            self.pixels.push(g);
            self.pixels.push(b);
            self.pixels.push(255);
        }

        Ok(())
    }

    fn save(&self, dir: &str) {
        let path = format!("{}/{}.png", dir, self.data.metadata.name_ascii());

        image::save_buffer(
            Path::new(&path),
            &self.pixels,
            FLAT_W as u32,
            FLAT_H as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }

    fn data(&self) -> LumpData {
        self.data.clone()
    }

    fn set_data(&mut self, data: LumpData) {
        self.data = data;
    }

    fn update(&mut self, _buffer: &Vec<u8>) {
        todo!()
    }
}
