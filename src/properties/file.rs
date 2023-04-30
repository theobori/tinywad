use std::{
    path::Path,
    fs::File,
    io::Read
};
use crate::error::WadError;

pub struct PathWrap<P: AsRef<Path>>(P);

impl Into<PathWrap<&str>> for &'static str {
    fn into(self) -> PathWrap<&'static str> {
        PathWrap(self)
    }
}

impl TryFrom<PathWrap<&str>> for Vec<u8> {
    type Error = WadError;

    fn try_from(value: PathWrap<&str>) -> Result<Self, Self::Error> {
        match File::open(value.0) {
            Ok(mut file) => {
                let mut data = Vec::<u8>::new();
    
                match file.read_to_end(data.as_mut()) {
                    Ok(_) => Ok(data),
                    Err(e) => Err(WadError::Read(e.to_string()))
                }
            },
            Err(e) => Err(WadError::Read(e.to_string()))
        }
    }
}
