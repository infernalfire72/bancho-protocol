use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

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