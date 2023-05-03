use std::path::Path;
use regex::Error;

use crate::{
    error::WadError,
    properties::file::PathWrap, lump::LumpAdd
};

/// Operate on the matching lumps
pub trait WadOp {
    /// Dump
    fn dump(&self);
    /// Extract as file(s) in the directory `dir`
    fn save_lumps<P: AsRef<Path>>(&self, dir: P);
    /// Extract the raw content in the directory `dir`
    fn save_lumps_raw<P: AsRef<Path>>(&self, dir: P);
    /// Remove a lump
    /// 
    /// Only affects `self.save`
    fn remove(&mut self);
    /// Remove lump(s) by its name
    /// 
    /// Only affects `self.save`
    fn remove_by_name(&mut self, re: &str) -> Result<(), Error>;
    /// Build then output the WAD content as a new file
    fn save<P: AsRef<Path>>(&mut self, path: P);
    /// Update a lump buffer
    fn update_lumps_raw(&mut self, buffer: &Vec<u8>);
    /// `self.update_lumps_raw` wrapper 
    fn update_lumps_raw_from_file<P: Into<PathWrap<&'static str>>>(
        &mut self,
        path: P
    ) -> Result<(), WadError> {
        let path = path.into();
        let buffer = path.try_into()?;

        self.update_lumps_raw(&buffer);

        Ok(())
    }
    /// Update lump from a buffer with its original format
    /// 
    /// As example, for a DOOM image lump, you could
    /// pass a `buffer` of an image (png, jpg, etc..) file
    fn update_lumps(&mut self, buffer: &Vec<u8>);
    /// `self.update_lumps` wrapper 
    fn update_lumps_from_file<P: Into<PathWrap<&'static str>>>(
        &mut self,
        path: P
    ) -> Result<(), WadError> {
        let path = path.into();
        let buffer = path.try_into()?;

        self.update_lumps(&buffer);

        Ok(())
    }

    /// Add lump from a raw buffer
    /// 
    /// We assume this method will be used only for build a new WAD
    /// and not for dump or extract
    fn add_lump_raw(&mut self, add: LumpAdd) -> Result<(), WadError>;
}
