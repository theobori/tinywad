use std::collections::HashMap;

use crate::error::WadError;

use super::mus::{
    Mus,
    MetaEvent,
    MusReleaseNote,
    MusPlayNote,
    MusSystemEvent,
    MusController,
    controller_as_midi
};

/// MID file controller
#[derive(Clone)]
pub struct Midi {
    /// Representing the raw buffer
    buffer: Vec<u8>
}

impl Default for Midi {
    fn default() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

impl Midi {
    pub fn new() -> Self {
        Self::default()
    }

    /// Write the MID file metadata
    fn write_metadata(&mut self) {
        // Header
        self.buffer.append(&mut vec![0x4d, 0x54, 0x68, 0x64]);
        self.buffer.append(&mut u32::to_be_bytes(6).to_vec());
        self.buffer.append(&mut u16::to_be_bytes(0).to_vec());
        self.buffer.append(&mut u16::to_be_bytes(1).to_vec());
        self.buffer.append(&mut u16::to_be_bytes(560).to_vec());

        // Track (only one) block
        self.buffer.append(&mut vec![0x4d, 0x54, 0x72, 0x6b]);
        self.buffer.append(
            &mut u32::to_be_bytes(0).to_vec()
        );

        // Tempo
        self.buffer.append(
            &mut vec![
                0,
                0xff,
                0x51,
                3,
                0x0f,
                0x42,
                0x40
            ]
        );
    }

    /// Borrows a buffer copy
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    /// Borrows a buffer copy
    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    /// Reset the buffer
    pub fn reset(&mut self) {
        self.buffer.clear()
    }
}

impl TryFrom<&Mus> for Midi {
    type Error = WadError;

    fn try_from(value: &Mus) -> Result<Self, Self::Error> {
        let mut midi = Midi::new();
        let mut delay: usize = 0;
        let mut channels: HashMap<u8, u8> = HashMap::new();
        
        midi.write_metadata();
        
        // File buffer
        let midi_buffer= midi.buffer_mut();
        // MUS Events buffer
        let event_buffer= value.event_buffer();
        let mut i = 0;
        
        while i < value.header().song_len as usize {
            // Read the MUS event
            let meta = MetaEvent(event_buffer[i]);
            i += 1;

            let channel = if meta.channel() == 15 {
                9
            } else {
                meta.channel()
            };

            let mid_delay = delay * 4;
            if mid_delay >= 0x200000 {
                midi_buffer.push(((mid_delay & 0xfe00000) >> 21 | 0x80) as u8);
            }
            if mid_delay >= 0x4000 {
                midi_buffer.push(((mid_delay & 0x1fc000) >> 14 | 0x80) as u8);
            }
            if mid_delay >= 0x80 {
                midi_buffer.push(((mid_delay & 0x3f80) >> 7 | 0x80) as u8);
            }
            midi_buffer.push(mid_delay as u8 & 0x7f);

            match meta.event_type() {
                0 => {
                    // println!("MusReleaseNote");
                    let event = MusReleaseNote(event_buffer[i]);
                    let volume = channels
                        .get(&channel)
                        .unwrap_or(&100);

                    // println!("volume {}", *volume);
                    midi_buffer.push(0x80 | channel);
                    midi_buffer.push(event.note());
                    midi_buffer.push(*volume);
                },
                1 => {
                    let event = MusPlayNote(
                        event_buffer[i],
                        event_buffer[i + 1]
                    );
    
                    let mut default_volume = 100;
                    let volume = channels
                        .get_mut(&channel)
                        .unwrap_or(&mut default_volume);

                    if event.is_volume() {
                        *volume = event.volume();
                    }

                    midi_buffer.push(0x90 | channel);
                    midi_buffer.push(event.note());
                    midi_buffer.push(*volume);

                    if event.is_volume() {
                        i += 1;
                    }
                },
                2 => {
                    midi_buffer.push(0xe0 | channel);
                    midi_buffer.push((event_buffer[i] << 7) & 0x80);
                    midi_buffer.push(event_buffer[i] >> 1);
                },
                3 => {
                    let event = MusSystemEvent(event_buffer[i]);
                    let controller = controller_as_midi(event.controller())?;

                    midi_buffer.push(0xb0 | channel);
                    midi_buffer.push(controller);
                    // Vritual controller value
                    midi_buffer.push(0);
                    
                },
                4 => {
                    let event = MusController(
                        event_buffer[i],
                        event_buffer[i + 1]
                    );
                    let controller = controller_as_midi(event.controller())?;

                    if event.controller() == 0 {
                        // Instrument change
                        midi_buffer.push(0xc0 | channel);
                        midi_buffer.push(event.value());
                    } else {
                        midi_buffer.push(0xb0 | channel);
                        midi_buffer.push(controller);
                        midi_buffer.push(event.value());
                    }

                    let volume = channels
                        .get(&channel)
                        .unwrap_or(&0);

                    if event.controller() == 3 {
                        channels.insert(channel,*volume);
                    }

                    i += 1;
                },
                5 => {},
                6 => {
                    midi_buffer.push(0xff);
                    midi_buffer.push(0x2f);
                    midi_buffer.push(0x80);
                }
                _ => {}
            }

            i += 1;

            if meta.last() == false {
                delay = 0;
                continue
            }

            let mut byte = 0x80;
            
            let mut tmp_delay = 0;
            while byte & 0x80 == 0x80 {
                byte = event_buffer[i];
                tmp_delay = tmp_delay * 128 + (byte & 0x7f) as usize;

                i += 1;
            }

            delay = tmp_delay;
        }

        // Overwrite the length
        let size = u32::to_be_bytes(midi_buffer.len() as u32 - 22);

        for (i, byte) in size.iter().enumerate() {
            midi_buffer[18 + i] = *byte;
        }

        Ok(midi)
    }
}
