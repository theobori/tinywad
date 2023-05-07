use crate::error::WadError;

/// .MUS format magic bytes
pub const MUS_MAGIC: [u8; 4] = [0x4d, 0x55, 0x53, 0x1a];

/// Represents the Music header
/// 
/// Every `u16` are in little-endianess
#[derive(Clone, Debug)]
pub struct MusHeader {
    /// .MUS file identifier "MUS" 0x1A
    pub magic: [u8; 4],
    /// Score length in bytes
    pub song_len: u16,
    /// Absolute file position of the score
    pub song_start: u16,
    /// Count of primary channels
    pub channels: u16,
    /// Count of secondary channels
    pub sec_channels: u16,
    /// Instrument count
    pub instr_count: u16,
    /// Reserved bytes
    pub dummy: u16,
    /// Variable-length part starts here
    pub instruments: Vec<u16>
}

impl Default for MusHeader {
    fn default() -> Self {
        Self {
            magic: [0, 0, 0, 0],
            song_len: 0,
            song_start: 0,
            channels: 0,
            sec_channels: 0,
            instr_count: 0,
            dummy: 0,
            instruments: Vec::new()
        }
    }
}

impl From<&[u8]> for MusHeader {
    fn from(value: &[u8]) -> Self {
        Self {
            magic: value[0..4]
                .try_into()
                .unwrap_or_default(),
            song_len: u16::from_le_bytes(
                value[4..6]
                    .try_into()
                    .unwrap_or_default()
            ),
            song_start: u16::from_le_bytes(
                value[6..8]
                    .try_into()
                    .unwrap_or_default()
                ),
            channels: u16::from_le_bytes(
                value[8..10]
                    .try_into()
                    .unwrap_or_default()
                ),
            sec_channels: u16::from_le_bytes(
                value[10..12]
                    .try_into()
                    .unwrap_or_default()
                ),
            instr_count: u16::from_le_bytes(
                value[12..14]
                    .try_into()
                    .unwrap_or_default()
                ),
            dummy: u16::from_le_bytes(
                value[14..16]
                    .try_into()
                    .unwrap_or_default()
                ),
            instruments: Vec::new(),
        }
    }
}

/// Provides informations about the event to perform
pub struct MetaEvent(pub u8);

impl MetaEvent {
    /// It is set when the event is followed by a delay
    pub fn last (&self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Indicates the event type
    pub fn event_type(&self) -> u8 {
        (self.0 & 0x70) >> 4
    }

    /// The channel the event is played on
    pub fn channel(&self) -> u8 {
        self.0 & 0xf
    }
}

/// Event 0
/// 
/// This event stops the given note playing on the channel
/// specified by the event
/// 
/// Other notes on the channel are left playing.
pub struct MusReleaseNote(pub u8);

impl MusReleaseNote {
    /// Note number
    pub fn note(&self) -> u8 {
        self.0 & 0x7f
    }
}

/// Event 1
/// 
/// Play the note
pub struct MusPlayNote(pub u8, pub u8);

impl MusPlayNote {
    /// Note number
    pub fn note(&self) -> u8 {
        self.0 & 0x7f
    }

    /// If the volume flag is set, it will use the
    /// volume stored in the next byte
    /// 
    /// Otherwise, it will use the volume of the previous note
    /// on the channel
    pub fn is_volume(&self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Get the volume
    pub fn volume(&self) -> u8 {
        self.1 & 0x7f
    }
}

/// Event 3
/// 
/// A system event is a controller with no associated value
/// The following values are valid, with their corresponding MIDI controller numbers:
/// 
/// See here for more details: https://moddingwiki.shikadi.net/wiki/MUS_Format
pub struct MusSystemEvent(pub u8);

impl MusSystemEvent {
    /// Returns the controller ID
    pub fn controller(&self) -> u8 {
        self.0 & 0x7f
    }
}

/// Event 4
/// 
/// A controller assigned to a value
/// 
/// See here for more details: https://moddingwiki.shikadi.net/wiki/MUS_Format
/// 
pub struct MusController(pub u8, pub u8);

impl MusController {
    /// Returns the controller ID
    pub fn controller(&self) -> u8 {
        self.0 & 0x7f
    }

    /// Returns the controller value
    pub fn value(&self) -> u8 {
        self.1 & 0x7f
    }
}

/// Represents a MUS file with tis metadata
#[derive(Clone)]
pub struct Mus {
    /// Header
    header: MusHeader,
    /// Raw events content
    event_buffer: Vec<u8>
}

impl Default for Mus {
    fn default() -> Self {
        Self {
            header: MusHeader::default(),
            event_buffer: Vec::new()
        }
    }
}

impl Mus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrows the header
    pub fn header(&self) -> &MusHeader {
        &self.header
    }

    /// Borrows a event_buffer copy
    pub fn event_buffer(&self) -> &Vec<u8> {
        &self.event_buffer
    }

    /// Borrows a event_buffer copy
    pub fn event_buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.event_buffer
    }
}

impl From<&[u8]> for Mus {
    fn from(value: &[u8]) -> Self {
        let header = MusHeader::from(value);

        let offset = (16 + (header.instr_count * 2)) as usize;
        let event_buffer = value[offset..].to_vec();

        Self {
            header,
            event_buffer
        }
    }
}

/// Take a value `value` that represents the MUS controller value
/// then convert it as a MIDI one
pub fn controller_as_midi(
    value: u8
) -> Result<u8, WadError> {
    let ret = match value {
        0 | 1 => 0,
        2 => 1,
        3 => 7,
        4 => 10,
        5 => 11,
        6 => 91,
        7 => 93,
        8 => 64,
        9 => 67,
        10 => 120,
        11 => 123,
        12 => 126,
        13 => 127,
        14 => 121,
        _ => return Err(WadError::InvalidLump)
    };

    Ok(ret)
}
