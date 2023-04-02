use std::path::Path;

/// Operate on the matching lumps
pub trait WadOperation {
    /// Dump
    fn dump(&self);
    /// Extract as file(s) in the directory `dir`
    fn save<P: AsRef<Path>>(&self, dir: P);
    /// Extract the raw content in the directory `dir`
    fn save_raw<P: AsRef<Path>>(&self, dir: P);
}
