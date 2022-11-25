use std::fmt::Display;

pub trait Lump: Display {
    /// parse the lump as a file if parseable
    fn parse(&mut self, buffer: &[u8]);
    /// Save the data as a file to the dir `dir`
    fn save_as(&self, dir: &str);
    /// Save the data as a file, the location is the current dir with the default lump name
    fn save(&self) {
        self.save_as(".");
    }
}
