use crate::serde::byte_sized::ByteSized;
use crate::serde::osu_types::PrefixedVec;
use crate::serde::{BinaryDeserialize, BinaryReader, BinarySerialize, BinaryWriter};
use crate::structures::ScoreFrame;
use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use bitflags::bitflags;
use std::io::{Error, ErrorKind};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum ReplayAction {
    Standard = 0,
    NewSong = 1,
    Skip = 2,
    Completion = 3,
    Fail = 4,
    Pause = 5,
    Unpause = 6,
    SongSelect = 7,
    WatchingOther = 8,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct ButtonState: u8 {
        const M1 = 1;
        const M2 = 2;
        const K1 = 4;
        const K2 = 8;
        const Smoke = 16;
    }
}

#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub struct ReplayFrame {
    pub button_state: ButtonState,
    pub taiko_byte: u8,
    pub x: f32,
    pub y: f32,
    pub time: i32,
}

#[derive(Debug, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub struct ReplayFrameBundle {
    pub extra: i32,
    pub frames: PrefixedVec<i16, ReplayFrame>,
    pub action: ReplayAction,
    pub score_frame: ScoreFrame,
    pub sequence: u16,
}

impl TryFrom<u8> for ReplayAction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 8 {
            Err(Error::new(ErrorKind::InvalidData, "invalid replay action"))
        } else {
            // SAFETY: `ReplayAction` has 9 variants
            Ok(unsafe { std::mem::transmute(value) })
        }
    }
}

impl<'a> BinaryDeserialize<'a> for ButtonState {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let val = u8::read_from(reader)?;
        Ok(ButtonState::from_bits_retain(val))
    }
}

impl ByteSized for ButtonState {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }
}

impl BinarySerialize for ButtonState {
    fn write_to(&self, writer: &mut BinaryWriter) {
        writer.write_byte(self.bits());
    }
}
