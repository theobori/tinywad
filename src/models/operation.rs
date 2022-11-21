use std::path::Path;

/// Operate on the matching lumps
pub trait WadOperation {
    /// Dump
    fn dump(&self);
    /// Extract as file(s)
    fn extract(&self);
    /// Extract as file(s) in the directory `path`
    fn extract_in<P: AsRef<Path>>(&self, path: P);
}
