use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use crate::serde::osu_types::PrefixedVec;
use crate::structures::ScoreFrame;

#[repr(u8)]
#[derive(Debug, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
#[allow(dead_code)]
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

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
#[allow(dead_code)]
pub enum ButtonState {
    M1,
    M2,
    K1 = 5,
    K2 = 10,
    Smoke = 16,
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