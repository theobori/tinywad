use std::{
    fs::{self},
    path::Path,
    str::FromStr,
};

use regex::{Error, Regex};

use crate::{
    dir::LumpsDirectory,
    error::WadError,
    lump::{LumpAdd, LumpData, LumpInfo, LumpKind},
    lumps::unknown::Unknown,
    models::{lump::Lump, operation::WadOp},
    output::WadOutput,
    properties::file::PathWrap,
};

/// Default re_name used by the `Wad` struct
pub const DEFAULT_RE_NAME: &'static str = r".*";
/// Iwad kind
pub const MAGIC_IWAD: &[u8] = &[0x49, 0x57, 0x41, 0x44];
/// Pwad kind
pub const MAGIC_PWAD: &[u8] = &[0x50, 0x57, 0x41, 0x44];

/// WAD types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WadKind {
    Iwad,
    Pwad,
    Unknown,
}

impl From<&[u8]> for WadKind {
    fn from(magic: &[u8]) -> Self {
        match magic {
            MAGIC_IWAD => Self::Iwad,
            MAGIC_PWAD => Self::Pwad,
            _ => Self::Unknown,
        }
    }
}

impl Into<Vec<u8>> for WadKind {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Iwad => MAGIC_IWAD.to_vec(),
            Self::Pwad => MAGIC_PWAD.to_vec(),
            Self::Unknown => vec![0x00; 4],
        }
    }
}

/// Operations kind
#[derive(Debug, Clone, Copy)]
pub enum WadOperationKind {
    Dump,
    Save,
    SaveAs,
}

impl Default for WadOperationKind {
    fn default() -> Self {
        Self::Dump
    }
}

impl FromStr for WadOperationKind {
    type Err = WadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "dump" => Self::Dump,
            "save" => Self::Save,
            "save_as" => Self::SaveAs,
            _ => return Err(WadError::InvalidOperation),
        };

        Ok(value)
    }
}

/// Wad metadata (header) (12 bytes)
#[derive(Clone, Copy)]
pub struct WadInfo {
    /// Represents the file type (4 bytes)
    pub kind: WadKind,
    /// Holding the lumps amount (4 bytes) (little-endian)
    pub num_lumps: i32,
    /// Holding pointer to the directory location (4 bytes) (little-endian)
    pub dir_pos: i32,
}

impl Default for WadInfo {
    fn default() -> Self {
        Self {
            kind: WadKind::Unknown,
            num_lumps: 0,
            dir_pos: 0,
        }
    }
}

impl From<&[u8]> for WadInfo {
    fn from(bytes: &[u8]) -> Self {
        Self {
            kind: WadKind::from(&bytes[0..4]),
            num_lumps: i32::from_le_bytes(bytes[4..8].try_into().unwrap_or_default()),
            dir_pos: i32::from_le_bytes(bytes[8..12].try_into().unwrap_or_default()),
        }
    }
}

impl Into<Vec<u8>> for WadInfo {
    fn into(self) -> Vec<u8> {
        let mut ret = Vec::new();

        ret.append(&mut self.kind.into());
        ret.append(&mut i32::to_le_bytes(self.num_lumps).to_vec());
        ret.append(&mut i32::to_le_bytes(self.dir_pos).to_vec());

        ret
    }
}
/// WAD controller, it includes features like extract, dump, etc..
pub struct Wad {
    /// File type (IWAD or PWAD)
    info: WadInfo,
    /// Buffer a.k.a the source file content
    src: Vec<u8>,
    /// Filter (regex)
    re_name: Regex,
    /// Lumps directory
    dir: LumpsDirectory,
}

impl Wad {
    pub fn new() -> Self {
        Self {
            info: WadInfo::default(),
            src: Vec::new(),
            re_name: Regex::new(DEFAULT_RE_NAME).unwrap(),
            dir: LumpsDirectory::new(),
        }
    }

    /// Set a palette that will be applied on every lump
    pub fn set_palette(&mut self, value: usize) {
        self.dir.set_palette(value);
    }

    /// Set the WAD kind (IWAD/PWAD/UNKOWN)
    pub fn set_kind(&mut self, value: WadKind) {
        self.info.kind = value;
    }

    /// Set `self.re_name`
    pub fn select(&mut self, value: &str) {
        let regex = Regex::new(value);

        self.re_name = match regex {
            Ok(r) => r,
            Err(_) => Regex::new(DEFAULT_RE_NAME).unwrap(),
        }
    }

    /// Parse a buffer into lumps entries
    pub fn load<T: Into<Vec<u8>>>(&mut self, buffer: T) -> Result<(), WadError> {
        let buffer = buffer.into();

        // WAD informations
        // Check if the WAD is valid
        if buffer.len() < 12 {
            return Err(WadError::Load("The file size is too small."));
        }

        self.info = WadInfo::from(&buffer[0..12]);

        if self.info.kind == WadKind::Unknown {
            return Err(WadError::Type("The file is not a WAD file."));
        }

        self.src = buffer;

        // Parse lumps
        self.dir.parse(self.info, &self.src)
    }

    /// Load file content from a path
    pub fn load_from_file<P: Into<PathWrap<&'static str>>>(
        &mut self,
        path: P,
    ) -> Result<(), WadError> {
        let path = path.into();
        let buffer: Vec<u8> = path.try_into()?;

        self.load(buffer)
    }

    /// Reparse the WAD
    ///
    /// Could be load after changing the palette index
    pub fn reload(&mut self) -> Result<(), WadError> {
        self.load(self.src.clone())
    }

    /// Build the entire WAD buffer based on its abstraction and `self.src`
    ///
    /// This method avoids us to write the changes directly on `self.src`,
    /// we are able to update or remove lumps without any problems.
    ///
    /// It will be called each time the user will save the entire WAD buffer
    fn dest(&mut self) -> Vec<u8> {
        let mut output = WadOutput::new(self.info, &self.dir);

        output.build();
        output.buffer()
    }

    /// Get a lump by its name
    pub fn lump(&self, name: &str) -> Option<&Box<dyn Lump>> {
        self.dir.lump(name)
    }
}

impl WadOp for Wad {
    fn dump(&self) {
        self.dir
            .callback_lumps(self.re_name.clone(), |lump| println!("{}", lump));
    }

    fn save_lumps<P: AsRef<Path>>(&self, dir: P) {
        let dir = dir.as_ref().to_str().unwrap();

        self.dir
            .callback_lumps(self.re_name.clone(), |lump| lump.save(dir));
    }

    fn save_lumps_raw<P: AsRef<Path>>(&self, dir: P) {
        let dir = dir.as_ref().to_str().unwrap();

        self.dir.callback_lumps(self.re_name.clone(), |lump| {
            let data = lump.data();
            let path = format!("{}/{}.raw", dir, data.metadata.id_ascii());

            fs::write(path, data.buffer).unwrap_or_default();
        });
    }

    fn remove_by_name(&mut self, re: &str) -> Result<(), Error> {
        let removed = self.dir.remove_lumps(Regex::new(re)?);

        self.info.num_lumps -= removed as i32;

        Ok(())
    }

    fn remove(&mut self) {
        let removed = self.dir.remove_lumps(self.re_name.clone());

        self.info.num_lumps -= removed as i32;
    }

    fn save<P: AsRef<Path>>(&mut self, path: P) {
        fs::write(path, self.dest()).unwrap_or_default();
    }

    fn update_lumps_raw(&mut self, buffer: &Vec<u8>) {
        self.dir.callback_lumps_mut(self.re_name.clone(), |lump| {
            let mut data = lump.data();

            data.buffer = buffer.to_vec();
            data.metadata.size = data.buffer.len() as i32;

            lump.set_data(data);
        });
    }

    fn update_lumps(&mut self, buffer: &Vec<u8>) {
        // TODO: update the metadata in the lump (size)

        self.dir
            .callback_lumps_mut(self.re_name.clone(), |lump| lump.update(buffer));
    }

    fn add_lump_raw(&mut self, add: LumpAdd) -> Result<(), WadError> {
        // A little bit hacky, it is just a way to get an unique position
        // it is useful for building a new WAD

        let pos = self
            .dir
            .lumps
            .iter()
            .map(|lump| lump.data().metadata.pos)
            .max()
            .unwrap_or(1);

        let metadata = LumpInfo::new(pos + 1, add.buffer.len() as i32, add.name);
        let unknown = Unknown {
            data: LumpData {
                buffer: add.buffer.clone(),
                metadata,
                kind: LumpKind::Unknown,
            },
        };

        // Lump informations
        let lump: Box<dyn Lump> = Box::new(unknown);
        let index = self.dir.index_from_kind(add.kind)?;

        self.dir.lumps.insert(index, lump);
        self.info.num_lumps += 1;

        Ok(())
    }
}
