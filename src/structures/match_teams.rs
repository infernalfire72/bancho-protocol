use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum MatchTeam {
    None,
    Blue,
    Red,
}

impl MatchTeam {
    pub fn from_u8(mut value: u8) -> Self {
        if value > 2 {
            value = 0;
        }

        match value {
            0 => MatchTeam::None,
            1 => MatchTeam::Blue,
            2 => MatchTeam::Red,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for MatchTeam {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(MatchTeam::None),
            1 => Ok(MatchTeam::Blue),
            2 => Ok(MatchTeam::Red),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                "invalid match team",
            )),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum MatchTeamType {
    HeadToHead,
    TagCoop,
    Vs,
    TagVs,
}

impl TryFrom<u8> for MatchTeamType {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(MatchTeamType::HeadToHead),
            1 => Ok(MatchTeamType::TagCoop),
            2 => Ok(MatchTeamType::Vs),
            3 => Ok(MatchTeamType::TagVs),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                "invalid team type",
            )),
        }
    }
}
