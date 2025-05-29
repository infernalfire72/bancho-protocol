use crate::serde::macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use crate::structures::{Mode, Mods};

#[repr(u8)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized,
)]
#[crate_root(crate)]
pub enum Action {
    #[default]
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

impl TryFrom<u8> for Action {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(Action::Idle),
            1 => Ok(Action::AFK),
            2 => Ok(Action::Playing),
            3 => Ok(Action::Editing),
            4 => Ok(Action::Modding),
            5 => Ok(Action::Multiplayer),
            6 => Ok(Action::Watching),
            7 => Ok(Action::Ranking),
            8 => Ok(Action::Testing),
            9 => Ok(Action::Submitting),
            10 => Ok(Action::Paused),
            11 => Ok(Action::Lobby),
            12 => Ok(Action::Multiplaying),
            13 => Ok(Action::Direct),
            _ => Err(Self::Error::new(ErrorKind::InvalidData, "invalid action")),
        }
    }
}

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct UserAction<'a> {
    pub action: Action,
    pub info_text: &'a str,
    pub beatmap_md5: &'a str,
    pub mods: Mods,
    pub mode: Mode,
    pub beatmap_id: i32,
}
