use std::str::FromStr;

use crate::error::WadError;

/// Header kinds used to preprocess a WAD build
/// 
/// Specially during the draining step
#[derive(PartialEq, Eq)]
pub enum HeaderKind {
    Delete,
    Update,
    Unknown
}

const DELETE_KIND: &str = "DELETE"; 
const UPDATE_KIND: &str = "UPDATE"; 
const UNKNOWN_KIND: &str = "UNKNOWN"; 

impl ToString for HeaderKind {
    fn to_string(&self) -> String {
        match self {
            HeaderKind::Delete => DELETE_KIND,
            HeaderKind::Update => UPDATE_KIND,
            _ => UNKNOWN_KIND,
        }
        .to_string()
    }
}

impl Into<Vec<u8>> for HeaderKind {
    fn into(self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

impl FromStr for HeaderKind {
    type Err = WadError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = match s {
            DELETE_KIND => Self::Delete,
            UPDATE_KIND => Self::Update,
            _ => Self::Unknown
        };

        Ok(ret)
    }
}

impl From<&[u8]> for HeaderKind {
    fn from(value: &[u8]) -> Self {
        let s = String::from_utf8_lossy(value);
        
        HeaderKind::from_str(&s).unwrap()
    }
}

/// Represents the header size
#[derive(Clone, Copy)]
pub struct HeaderSize(i32);

impl From<&[u8]> for HeaderSize {
    fn from(value: &[u8]) -> Self {
        Self(
            i32::from_le_bytes(
                value[0..4]
                    .try_into()
                    .unwrap_or_default()
            )
        )
    }
}

impl From<i32> for HeaderSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Into<Vec<u8>> for HeaderSize {
    fn into(self) -> Vec<u8> {
        i32::to_le_bytes(self.0).to_vec()
    }
}

/// Header containing meatadata for parsing
pub struct Header {
    /// Header kind
    pub kind: HeaderKind,
    /// Size of the content (includes the header itself)
    size: HeaderSize
}

impl Header {
    pub fn new(kind: HeaderKind, size: i32) -> Self {
        Self {
            kind,
            size: HeaderSize(size),
        }
    }

    /// Returns the header size
    pub fn size(&self) -> i32 {
        self.size.0
    }

    /// Returns the size as a bytes Vector
    /// 
    /// Endianness: little
    pub fn size_as_vec(&self) -> Vec<u8> {
        self.size.into()
    }

    /// Set `self.size` with a value that implements `T`
    pub fn set_size<T: Into<HeaderSize>>(&mut self, value: T) {
        self.size = value.into();
    }
}

impl From<&[u8]> for Header {
    fn from(value: &[u8]) -> Self {
        // We assume every kind has the same length
        let sep = DELETE_KIND.len();
    
        Self {
            kind: HeaderKind::from(&value[..sep]),
            size: HeaderSize::from(&value[sep..])
        }
    }
}

impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        let mut ret = Vec::new();
        let mut size = self.size_as_vec();

        ret.append(&mut self.kind.into());
        ret.append(&mut size);

        ret
    }
}