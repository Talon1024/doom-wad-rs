use crate::wad::DoomWadLump;
use binrw::BinRead;
use thiserror::Error;
use std::io::Cursor;
#[cfg(feature = "hound")]
use hound::{WavSpec, SampleFormat};

pub struct DMXSound<'wad> {
    lump: &'wad DoomWadLump
}

#[derive(Debug, Clone, Default)]
pub struct Sound {
    pub sample_rate: u16,
    pub samples: Vec<u8>,
}

#[cfg(feature = "hound")]
impl Sound {
    pub fn spec(&self) -> WavSpec {
        WavSpec {
            channels: 1,
            sample_rate: self.sample_rate as u32,
            bits_per_sample: 8,
            sample_format: SampleFormat::Int
        }
    }
}

// DMX format sounds and PC speaker sounds are documented on DoomWiki:
// https://doomwiki.org/wiki/Sound
#[derive(BinRead)]
#[brw(little)]
struct DMXSoundBinary {
    #[br(assert(
        _format_number == 3,
        "Wrong format number {_format_number} (must be 3)",
    ))]
    _format_number: u16,
    sample_rate: u16,
    _sample_count: u32,
    _pre_padding: [u8; 16],
    #[br(count = _sample_count)]
    samples: Vec<u8>,
    _post_padding: [u8; 16],
}

#[derive(Debug, Error)]
pub enum SoundError {
    #[error("BinRead error: {0}")]
    BinReadError(binrw::Error),
}

impl From<binrw::Error> for SoundError {
    fn from(value: binrw::Error) -> Self {
        SoundError::BinReadError(value)
    }
}

impl<'wad> DMXSound<'wad> {
    pub fn to_sound(&self) -> Result<Sound, SoundError> {
        let sound_data = DMXSoundBinary::read(&mut Cursor::new(&self.lump.data))?;
        Ok(Sound {
            sample_rate: sound_data.sample_rate,
            samples: sound_data.samples,
        })
    }
}
