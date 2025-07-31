use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use std::io::Error;

#[repr(u8)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized,
)]
#[crate_root(crate)]
pub enum Mode {
    #[default]
    Standard,
    Taiko,
    Catch,
    Mania,
}

impl Mode {
    pub fn from_np(np_mode: &str) -> Option<Mode> {
        match np_mode {
            "Taiko" => Some(Mode::Taiko),
            "CatchTheBeat" => Some(Mode::Catch),
            "osu!mania" => Some(Mode::Mania),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(mut value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            value = 0;
        }

        // SAFETY: `Mode` has 4 variants
        Ok(unsafe { std::mem::transmute(value) })
    }
}
