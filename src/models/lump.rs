use std::fmt::Display;

use crate::lump::LumpData;

pub trait Lump: Display {
    /// parse the lump as a file if parseable
    fn parse(&mut self);
    /// Save the data as a file to the dir `dir`
    fn save(&self, dir: &str);
    /// Get the lump data
    fn data(&self) -> LumpData;
    /// Set the lump data
    fn set_data(&mut self, data: LumpData);
    /// Update the lump buffer from a buffer based on its original format
    /// like PNG, WAV, etc...
    /// 
    /// So `buffer` technically represents a file
    fn update(&mut self, buffer: &Vec<u8>);
}
