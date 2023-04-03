use std::fmt::Display;

use crate::lump::LumpData;

pub trait Lump: Display {
    /// parse the lump as a file if parseable
    fn parse(&mut self);
    /// Save the data as a file to the dir `dir`
    fn save(&self, dir: &str);
    /// Get the lump data
    fn data(&self) -> LumpData;
}
