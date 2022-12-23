/// Lumps kind implementing the `Lump` trait
#[derive(Clone, PartialEq, Copy)]
pub enum LumpKind {
    Flat,
    Patch,
    Sound,
    /// Unidentified lump
    Unknown
}

impl Default for LumpKind {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Represents the lump metadata (16 bytes)
#[derive(Clone, Copy)]
pub struct LumpInfo {
    /// The lump start position in the file buffer (4 bytes)
    pub pos: i32,
    /// The lump size in bytes (4 bytes)
    pub size: i32,
    /// Lump name, it only contains in theory [A-Z][0-9] (8 bytes)
    pub name: [u8; 8]
}

impl Default for LumpInfo {
    fn default() -> Self {
        Self {
            pos: 0,
            size: 0,
            name: [0x00; 8]
        }
    }
}

impl LumpInfo {
    /// Get the lump name as String
    pub fn name(&self) -> String {
        String::from_utf8(self.name.to_vec()).unwrap()
    }

    /// Get the name filtered by ascii characters only
    pub fn name_ascii(&self) -> String {
        let name = self.name().clone();

        name.chars().filter(
            | c | {
                c.is_ascii_alphanumeric() || c.is_ascii_punctuation()
            }
        ).collect()
    }
}

impl From<&[u8]> for LumpInfo {
    fn from(bytes: &[u8]) -> Self {
        Self {
            pos: i32::from_le_bytes(
                bytes[0..4]
                    .try_into()
                    .unwrap_or_default()
            ),
            size: i32::from_le_bytes(
                bytes[4..8]
                    .try_into()
                    .unwrap_or_default()
            ),
            name: bytes[8..16]
                .try_into()
                .unwrap_or_default(),
        }
    }
}
