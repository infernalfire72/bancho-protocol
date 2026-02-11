use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
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
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized, Eq, PartialEq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // MatchTeam tests
    #[test]
    fn test_match_team_none() {
        let team = MatchTeam::None;
        assert_eq!(team as u8, 0);
    }

    #[test]
    fn test_match_team_blue() {
        let team = MatchTeam::Blue;
        assert_eq!(team as u8, 1);
    }

    #[test]
    fn test_match_team_red() {
        let team = MatchTeam::Red;
        assert_eq!(team as u8, 2);
    }

    #[test]
    fn test_match_team_copy_clone() {
        let t1 = MatchTeam::Blue;
        let t2 = t1;
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_match_team_equality() {
        assert_eq!(MatchTeam::None, MatchTeam::None);
        assert_ne!(MatchTeam::None, MatchTeam::Blue);
        assert_ne!(MatchTeam::Blue, MatchTeam::Red);
    }

    #[test]
    fn test_match_team_from_u8_valid() {
        assert_eq!(MatchTeam::from_u8(0), MatchTeam::None);
        assert_eq!(MatchTeam::from_u8(1), MatchTeam::Blue);
        assert_eq!(MatchTeam::from_u8(2), MatchTeam::Red);
    }

    #[test]
    fn test_match_team_from_u8_out_of_range() {
        assert_eq!(MatchTeam::from_u8(3), MatchTeam::None);
        assert_eq!(MatchTeam::from_u8(255), MatchTeam::None);
    }

    #[test]
    fn test_match_team_try_from_valid() {
        assert_eq!(MatchTeam::try_from(0).unwrap(), MatchTeam::None);
        assert_eq!(MatchTeam::try_from(1).unwrap(), MatchTeam::Blue);
        assert_eq!(MatchTeam::try_from(2).unwrap(), MatchTeam::Red);
    }

    #[test]
    fn test_match_team_try_from_invalid() {
        assert!(MatchTeam::try_from(3).is_err());
        assert!(MatchTeam::try_from(255).is_err());
    }

    #[test]
    fn test_match_team_debug_format() {
        let team = MatchTeam::Blue;
        let debug_str = format!("{:?}", team);
        assert_eq!(debug_str, "Blue");
    }

    // MatchTeamType tests
    #[test]
    fn test_match_team_type_head_to_head() {
        let ttype = MatchTeamType::HeadToHead;
        assert_eq!(ttype as u8, 0);
    }

    #[test]
    fn test_match_team_type_tag_coop() {
        let ttype = MatchTeamType::TagCoop;
        assert_eq!(ttype as u8, 1);
    }

    #[test]
    fn test_match_team_type_vs() {
        let ttype = MatchTeamType::Vs;
        assert_eq!(ttype as u8, 2);
    }

    #[test]
    fn test_match_team_type_tag_vs() {
        let ttype = MatchTeamType::TagVs;
        assert_eq!(ttype as u8, 3);
    }

    #[test]
    fn test_match_team_type_copy_clone() {
        let t1 = MatchTeamType::Vs;
        let t2 = t1;
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_match_team_type_equality() {
        assert_eq!(MatchTeamType::HeadToHead, MatchTeamType::HeadToHead);
        assert_ne!(MatchTeamType::HeadToHead, MatchTeamType::TagCoop);
        assert_ne!(MatchTeamType::Vs, MatchTeamType::TagVs);
    }

    #[test]
    fn test_match_team_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(MatchTeamType::HeadToHead);
        set.insert(MatchTeamType::Vs);
        assert!(set.contains(&MatchTeamType::HeadToHead));
        assert!(set.contains(&MatchTeamType::Vs));
        assert!(!set.contains(&MatchTeamType::TagCoop));
    }

    #[test]
    fn test_match_team_type_try_from_valid() {
        assert_eq!(MatchTeamType::try_from(0).unwrap(), MatchTeamType::HeadToHead);
        assert_eq!(MatchTeamType::try_from(1).unwrap(), MatchTeamType::TagCoop);
        assert_eq!(MatchTeamType::try_from(2).unwrap(), MatchTeamType::Vs);
        assert_eq!(MatchTeamType::try_from(3).unwrap(), MatchTeamType::TagVs);
    }

    #[test]
    fn test_match_team_type_try_from_invalid() {
        assert!(MatchTeamType::try_from(4).is_err());
        assert!(MatchTeamType::try_from(255).is_err());
    }

    #[test]
    fn test_match_team_type_debug_format() {
        let ttype = MatchTeamType::TagVs;
        let debug_str = format!("{:?}", ttype);
        assert_eq!(debug_str, "TagVs");
    }

    #[test]
    fn test_match_team_type_all_variants() {
        let types = [
            MatchTeamType::HeadToHead,
            MatchTeamType::TagCoop,
            MatchTeamType::Vs,
            MatchTeamType::TagVs,
        ];
        for (i, ttype) in types.iter().enumerate() {
            assert_eq!(*ttype as u8, i as u8);
        }
    }

    // Serde roundtrip tests for MatchTeam
    #[test]
    fn test_match_team_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize, BinaryReader};
        for team in [MatchTeam::None, MatchTeam::Blue, MatchTeam::Red] {
            let bytes = team.serialize();
            let decoded = MatchTeam::deserialize(&bytes).unwrap();
            assert_eq!(team, decoded);

            // Also exercise read_from directly
            let mut reader = BinaryReader::from(bytes.as_slice());
            let decoded2 = MatchTeam::read_from(&mut reader).unwrap();
            assert_eq!(team, decoded2);
        }
    }

    // Serde roundtrip tests for MatchTeamType
    #[test]
    fn test_match_team_type_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize, BinaryReader};
        for ttype in [MatchTeamType::HeadToHead, MatchTeamType::TagCoop, MatchTeamType::Vs, MatchTeamType::TagVs] {
            let bytes = ttype.serialize();
            let decoded = MatchTeamType::deserialize(&bytes).unwrap();
            assert_eq!(ttype, decoded);

            // Also exercise read_from directly
            let mut reader = BinaryReader::from(bytes.as_slice());
            let decoded2 = MatchTeamType::read_from(&mut reader).unwrap();
            assert_eq!(ttype, decoded2);
        }
    }

    #[test]
    fn test_match_team_byte_size() {
        use crate::serde::byte_sized::ByteSized;
        assert_eq!(MatchTeam::None.byte_size(), 1);
        assert_eq!(MatchTeamType::HeadToHead.byte_size(), 1);
    }

    #[test]
    fn test_match_team_deserialize_invalid() {
        use crate::serde::BinaryDeserialize;
        let data = [3u8]; // invalid: max valid is 2
        assert!(MatchTeam::deserialize(&data).is_err());
    }

    #[test]
    fn test_match_team_type_deserialize_invalid() {
        use crate::serde::BinaryDeserialize;
        let data = [4u8]; // invalid: max valid is 3
        assert!(MatchTeamType::deserialize(&data).is_err());
    }

    #[test]
    fn test_match_team_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(MatchTeam::deserialize(&data).is_err());
    }

    #[test]
    fn test_match_team_type_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(MatchTeamType::deserialize(&data).is_err());
    }
}
