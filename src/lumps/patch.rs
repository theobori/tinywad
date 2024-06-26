use std::{
    fmt::{Display, Error},
    mem::size_of,
    ops::Mul,
    path::Path,
};

use crate::{error::WadError, lump::LumpData, lumps::palette::Palettes, models::lump::Lump};

extern crate image;

/// DOOM picture informations
#[derive(Clone, Copy)]
pub struct DoomImageInfo {
    /// Image width
    pub width: u16,
    /// Image height
    pub height: u16,
    /// Image left
    pub left: u16,
    /// Image top
    pub top: u16,
}

impl Default for DoomImageInfo {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            left: 0,
            top: 0,
        }
    }
}

impl From<&[u8]> for DoomImageInfo {
    fn from(bytes: &[u8]) -> Self {
        Self {
            width: u16::from_le_bytes(bytes[0..2].try_into().unwrap_or_default()),
            height: u16::from_le_bytes(bytes[2..4].try_into().unwrap_or_default()),
            left: u16::from_le_bytes(bytes[4..6].try_into().unwrap_or_default()),
            top: u16::from_le_bytes(bytes[6..8].try_into().unwrap_or_default()),
        }
    }
}

/// Represents a DOOM picture
#[derive(Clone)]
pub struct DoomImage {
    /// Picture metadata
    pub img_info: DoomImageInfo,
    /// Array used to store the DOOM image data before converting it into bitmap
    pixels: Vec<Option<u8>>,
    /// Attached palettes
    palettes: Palettes,
    /// Lump data
    data: LumpData,
}

impl DoomImage {
    pub fn new(palettes: Palettes, data: LumpData) -> Self {
        Self {
            img_info: DoomImageInfo::default(),
            pixels: Vec::new(),
            palettes,
            data,
        }
    }

    /// Get the final image buffer, structured as a RGBA format
    fn buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let palette = self.palettes.palette().unwrap();

        let (mut r, mut g, mut b, mut a): (u8, u8, u8, u8);

        for byte in self.pixels.iter() {
            if byte.is_none() {
                (r, g, b, a) = (0, 0, 0, 0);
            } else {
                (r, g, b, a) = palette[byte.unwrap() as usize].into();
            }

            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
            buffer.push(a);
        }

        buffer
    }
}

impl Display for DoomImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Width: {}, Height: {}",
            self.data.metadata.name_ascii(),
            self.data.metadata.size,
            self.data.metadata.pos,
            self.img_info.width,
            self.img_info.height
        )
    }
}

impl Lump for DoomImage {
    fn parse(&mut self) -> Result<(), WadError> {
        let buffer = &*self.data.buffer;
        self.img_info = DoomImageInfo::from(buffer);

        let img_size = (self.img_info.width as usize).mul(self.img_info.height as usize);
        let mut columns = Vec::new();

        // Default background value is the last color in the palette
        self.pixels = vec![None; img_size];

        // Filling columns
        for i in 0..self.img_info.width as usize {
            let pos = (i * 4) + size_of::<DoomImageInfo>();

            if pos + 4 >= buffer.len() {
                return Err(WadError::InvalidLump);
            }

            let value =
                i32::from_le_bytes(buffer[pos..pos + 4].try_into().unwrap_or_default()) as usize;

            columns.push(value);
        }

        #[allow(unused)]
        let mut pos = 0;
        #[allow(unused)]
        let mut pixel_count = 0;

        for i in 0..self.img_info.width as usize {
            pos = columns[i];

            let mut row_start = 0;

            while row_start != 0xff {
                row_start = buffer[pos];
                pos += 1;

                if row_start == 0xff {
                    break;
                }

                pixel_count = buffer[pos];
                pos += 2;

                for j in 0..pixel_count as usize {
                    let index = (((row_start as usize) + j) * self.img_info.width as usize) + i;
                    self.pixels[index] = Some(buffer[pos]);
                    pos += 1;
                }

                pos += 1;
            }
        }

        Ok(())
    }

    fn save(&self, dir: &str) {
        let path = format!("{}/{}.png", dir, self.data.metadata.name_ascii());

        image::save_buffer(
            Path::new(&path),
            &self.buffer(),
            self.img_info.width as u32,
            self.img_info.height as u32,
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
