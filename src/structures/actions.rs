use crate::structures::{Mod, Mode};
use crate::serde::macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub enum Action {
    Idle,
    AFK,
    Playing,
    Editing,
    Modding,
    Multiplayer,
    Watching,
    Ranking,
    Testing,
    Submitting,
    Paused,
    Lobby,
    Multiplaying,
    Direct,
}

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct UserAction<'a> {
    pub action: Action,
    pub info_text: &'a str,
    pub beatmap_md5: &'a str,
    pub mods: Mod,
    pub mode: Mode,
    pub beatmap_id: i32,
}