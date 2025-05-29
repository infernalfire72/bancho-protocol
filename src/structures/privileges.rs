use crate::serde::byte_sized::ByteSized;
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
