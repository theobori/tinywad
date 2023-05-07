use std::{
    fmt::{
        Display,
        Result
    }, fs,
};

use crate::{
    models::lump::Lump,
    lump::LumpData,
};

use super::{
    mus::Mus,
    mid::Midi
};

/// Represents a DOOM music
#[derive(Clone)]
pub struct DoomMusic {
    /// MUS
    mus: Mus,
    /// Lump data
    data: LumpData,
    /// MIDI
    midi: Option<Midi>
}

impl DoomMusic {
    pub fn new(data: LumpData) -> Self {
        Self {
            mus: Mus::new(),
            data,
            midi: None

        }
    }
}

impl Display for DoomMusic {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Name: {}, Size: {}, Offset: {}, Channels {}, {}, Instruments {}",
            self.data.metadata.id_ascii(),
            self.data.metadata.size,
            self.data.metadata.pos,
            self.mus.header().channels,
            self.mus.header().sec_channels,
            self.mus.header().instr_count
        )
    }
}

impl Lump for DoomMusic {
    fn parse(&mut self) {
        let buffer: &[u8] = &self.data.buffer;

        // Parsing as a MUS lump
        self.mus = buffer.into();

        // Midi::try_from(&self.mus).unwrap();

        // Parsing into the MIDI format
        if let Ok(value) = Midi::try_from(&self.mus) {
            self.midi = Some(value);
        }
    }

    fn save(&self, dir: &str) {
        if self.midi.is_none() {
            return
        }

        let midi = self.midi.as_ref().unwrap();
        let path = format!(
            "{}/{}.mid",
            dir,
            self.data.metadata.id_ascii()
        );
        
        fs::write(
            path,
            midi.buffer()
        ).unwrap_or_default();
    }

    fn data(&self) -> LumpData {
        self.data.clone()
    }

    fn set_data(&mut self, data: LumpData) {
        self.data = data;
    }

    fn update(&mut self, _buffer: &Vec<u8>) {
        todo!()
    }
}
