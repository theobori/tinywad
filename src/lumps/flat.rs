use std::{
    fmt::{
        Display,
        Result
    },
    path::Path,
    rc::Rc,
    cell::RefCell
};

use crate::{
    models::lump::Lump,
    lump::LumpInfo,
    lumps::palette::Palettes,
    save_raw
};

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
    /// Lump metadata
    pub info: LumpInfo,
    /// Array used to store the DOOM image data before converting it into bitmap
    pixels: Vec<u8>,
    /// Attached palettes
    palettes: Palettes,
    /// Raw file buffer
    raw: Rc<RefCell<Vec<u8>>>
}

impl Flat {
    pub fn new(
        info: LumpInfo,
        palettes: Palettes,
        raw: Rc<RefCell<Vec<u8>>>
    ) -> Self {
        Self {
            info,
            pixels: Vec::new(),
            palettes,
            raw
        }
    }
}

impl Display for Flat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Width: {}, Height: {}",
            self.info.name_ascii(),
            self.info.size,
            self.info.pos,
            FLAT_W,
            FLAT_H
        )
    }
}

impl Lump for Flat {
    fn parse(&mut self) {
        let buffer = &self.raw.borrow_mut()[self.info.pos as usize..];
        let palette = self.palettes
            .palette()
            .unwrap();
        
        for i in 0..FLAT_SIZE {
            let byte = buffer[i];
            let (r, g, b, _) = palette[byte as usize].into();
        
            self.pixels.push(r);
            self.pixels.push(g);
            self.pixels.push(b);
            self.pixels.push(255);
        }
    }        

    fn save(&self, dir: &str) {
        let path = format!(
            "{}/{}.png",
            dir,
            self.info.name_ascii()
        );

        image::save_buffer(
            Path::new(&path),
            &self.pixels,
            FLAT_W as u32,
            FLAT_H as u32,
            image::ColorType::Rgba8
        ).unwrap();
    }

    save_raw!();
}
