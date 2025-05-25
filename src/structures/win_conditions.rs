use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum WinCondition {
    Score,
    Accuracy,
    Combo,
    ScoreV2,
}

impl TryFrom<u8> for WinCondition {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(WinCondition::Score),
            1 => Ok(WinCondition::Accuracy),
            2 => Ok(WinCondition::Combo),
            3 => Ok(WinCondition::ScoreV2),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                "invalid win condition",
            )),
        }
    }
}
