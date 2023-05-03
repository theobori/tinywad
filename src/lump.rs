/// Lumps kind implementing the `Lump` trait
#[derive(Clone, PartialEq, Copy)]
pub enum LumpKind {
    Flat,
    Sound,
    Patch,
    Palette,
    /// Unidentified lump
    Unknown
}

impl Default for LumpKind {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Represents the lump state
/// 
/// It permits to organize the WAD operations (from `WadOp`)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LumpState {
    Default,
    Deleted,
    Updated
}

impl LumpState {
    /// Get the alive lumps amount
    pub fn is_alive(&self) -> bool{
        *self != Self::Deleted
    }
}

/// Represents the lump official metadata (16 bytes)
/// and the WAD operations state (from `WadOp`)
#[derive(Clone, Copy)]
pub struct LumpInfo {
    /// The lump start position in the file buffer (4 bytes)
    pub pos: i32,
    /// The lump size in bytes (4 bytes)
    pub size: i32,
    /// Lump name, it only contains in theory [A-Z][0-9] (8 bytes)
    pub name: [u8; 8],
    /// Lumps name are not unique but we want to extract them into files
    /// 
    /// The attribute acts like an unique ID
    pub id: [u8; 12],
    /// Lump state
    pub state: LumpState
}

impl Default for LumpInfo {
    fn default() -> Self {
        Self {
            pos: 0,
            size: 0,
            name: [0x00; 8],
            id: [0x00; 12],
            state: LumpState::Default
        }
    }
}

impl LumpInfo {
    pub fn new(
        pos: i32,
        size: i32,
        name: [u8; 8]
    ) -> Self {

        let mut id = [0x00; 12];

        for i in 0..name.len() {
            id[i] = name[i];
        }

        Self {
            pos,
            size,
            name,
            id,
            state: LumpState::Default
        }
    }
    /// Filter `value` with ascii characters
    fn ascii(value: String) -> String {
        value.chars().filter(
            | c | {
                c.is_ascii_alphanumeric() || c.is_ascii_punctuation()
            }
        ).collect()
    }
    
    /// Get the lump name as String
    pub fn name(&self) -> String {
        String::from_utf8(self.name.to_vec()).unwrap()
    }

    /// Get the lump unique ID as String
    pub fn id(&self) -> String {
        String::from_utf8(self.id.to_vec()).unwrap()
    }

    /// Get the lump unique ID filtered by ascii characters only
    pub fn id_ascii(&self) -> String {
        Self::ascii(self.id())
    }

    /// Get the name filtered by ascii characters only
    pub fn name_ascii(&self) -> String {
        Self::ascii(self.name())
    }

    /// Returns if the metadata is overwritable
    /// 
    /// Because specials lumps cannot be overwritten
    /// when we build a new WAD file (output)
    /// like the separators
    pub fn is_overwritable(&self) -> bool {
        self.state.is_alive() == true &&
        self.pos > 0 &&
        self.size > 0
    }
}

impl From<&[u8]> for LumpInfo {
    fn from(bytes: &[u8]) -> Self {
        let name: [u8; 8] = bytes[8..16]
            .try_into()
            .unwrap_or_default();
        let mut id = [0x00; 12];

        for i in 0..name.len() {
            id[i] = name [i];
        }

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
            name,
            id,
            state: LumpState::Default
        }
    }
}

impl Into<Vec<u8>> for LumpInfo {
    fn into(self) -> Vec<u8> {
        let mut ret = Vec::new();

        ret.append(&mut i32::to_le_bytes(self.pos).to_vec());
        ret.append(&mut i32::to_le_bytes(self.size).to_vec());
        ret.append(&mut self.name.to_vec());

        ret
    }
}

/// Representing the whole lump buffer as a struct
#[derive(Clone)]
pub struct LumpData {
    /// The raw buffer
    pub buffer: Vec<u8>,
    /// The lump metadata (16 bytes header)
    pub metadata: LumpInfo,
    /// The lump kind
    pub kind: LumpKind
}

impl Default for LumpData {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            metadata: Default::default(),
            kind: LumpKind::Unknown
        }
    }
}

impl Into<Vec<u8>> for LumpData {
    fn into(self) -> Vec<u8> {
        let mut ret = self.buffer;

        ret.append(
            &mut self.metadata.pos
                .to_be_bytes()
                .to_vec()
        );
        ret.append(
            &mut self.metadata.size
                .to_be_bytes()
                .to_vec()
        );
        ret.append(&mut self.metadata.name.to_vec());

        ret
    }
}

/// Every kind for `LumpAdd`
pub enum LumpAddKind {
    /// After a lump (lump name)
    After(&'static str),
    /// Before a lump (lump name)
    Before(&'static str),
    /// It add the lump to the start
    /// 
    /// *Not recommended for an IWAD*
    Front,
    /// Add the lump to the end
    Back
}

/// Metadata for an adding lump operation
pub struct LumpAdd<'a> {
    /// Kind of adding
    pub kind: LumpAddKind,
    /// Lump raw buffer
    pub buffer: &'a Vec<u8>,
    /// Lump name
    pub name: [u8; 8]
}

impl<'a> LumpAdd<'a> {
    pub fn new(
        kind: LumpAddKind,
        buffer: &'a Vec<u8>,
        name: &str
    ) -> Self {
        let mut array = [0; 8];
        let mut i = 0;
        let bytes = name.as_bytes();

        while i < array.len() && i < bytes.len() {
            array[i] = bytes[i];

            i += 1;
        }

        Self {
            kind,
            buffer,
            name: array
        }
    }
}
