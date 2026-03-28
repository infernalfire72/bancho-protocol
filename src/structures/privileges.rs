use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug, Copy, Clone, PartialEq)]
    pub struct Privileges: u32 {
        const None = 0;
        const Player = 1;
        const Moderator = 2;
        const Supporter = 4;
        const LeGuy = 8;
        const Developer = 16;
        const TournamentStaff = 32;
    }
}

impl ByteSized for Privileges {
    fn byte_size(&self) -> usize {
        size_of::<u32>()
    }
}

impl BinarySerialize for Privileges {
    fn write_to(&self, writer: &mut BinaryWriter) {
        u32::write_to(&self.bits(), writer)
    }
}

impl<'a> BinaryDeserialize<'a> for Privileges {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let bits = u32::read_from(reader)?;
        Ok(Privileges::from_bits_retain(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privileges_byte_size() {
        let privs = Privileges::None;
        assert_eq!(privs.byte_size(), 4);
    }

    #[test]
    fn test_privileges_byte_size_combined() {
        let privs = Privileges::Player | Privileges::Supporter;
        assert_eq!(privs.byte_size(), 4);
    }

    #[test]
    fn test_privileges_serialize_none() {
        let privs = Privileges::None;
        let bytes = privs.serialize();
        assert_eq!(bytes, 0u32.to_le_bytes());
    }

    #[test]
    fn test_privileges_serialize_player() {
        let privs = Privileges::Player;
        let bytes = privs.serialize();
        assert_eq!(bytes, 1u32.to_le_bytes());
    }

    #[test]
    fn test_privileges_serialize_combined_flags() {
        let privs = Privileges::Player | Privileges::Moderator | Privileges::Supporter;
        let bytes = privs.serialize();
        // 1 | 2 | 4 = 7
        assert_eq!(bytes, 7u32.to_le_bytes());
    }

    #[test]
    fn test_privileges_serialize_deserialize_roundtrip() {
        let original = Privileges::Player | Privileges::Developer;
        let bytes = original.serialize();
        let deserialized = Privileges::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, original);
    }

    #[test]
    fn test_privileges_deserialize_all_flags() {
        let all = Privileges::Player
            | Privileges::Moderator
            | Privileges::Supporter
            | Privileges::LeGuy
            | Privileges::Developer
            | Privileges::TournamentStaff;
        let bytes = all.serialize();
        let deserialized = Privileges::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, all);
    }
}

