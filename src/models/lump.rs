use std::fmt::Display;

pub trait Lump: Display {
    /// parse the lump as a file if parseable
    fn parse(&mut self);
    /// Save the data as a file to the dir `dir`
    fn save(&self, dir: &str);
    /// Save the data as a raw file to the dir `dir`
    fn save_raw(&self, dir: &str);
}

#[macro_export]
macro_rules! save_raw {
    () => {
        fn save_raw(&self, dir: &str) {
            let path = format!(
                "{}/{}.raw",
                dir,
                self.info.name_ascii()
            );

            let pos = self.info.pos as usize;
            let size = self.info.size as usize;
            
            std::fs::write(
                path,
                &self.raw.borrow_mut()[pos..pos + size]
            ).unwrap_or_default();
        }
    };
}

pub use save_raw;
