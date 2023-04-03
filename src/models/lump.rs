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

// macro_rules! save_raw {
//     () => {
//         fn save_raw(&self, dir: &str) {
//             let path = format!(
//                 "{}/{}.raw",
//                 dir,
//                 self.info.name_ascii()
//             );

//             let pos = self.info.pos as usize;
//             let size = self.info.size as usize;
            
//             std::fs::write(
//                 path,
//                 &self.raw.borrow_mut()[pos..pos + size]
//             ).unwrap_or_default();
//         }
//     };
// }

// pub use save_raw;

