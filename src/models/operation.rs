use std::path::Path;

/// Operate on the matching lumps
pub trait WadOperation {
    /// Dump
    fn dump(&self);
    /// Extract as file(s)
    fn save(&self);
    /// Extract as file(s) in the directory `path`
    fn save_as<P: AsRef<Path>>(&self, path: P);
}
