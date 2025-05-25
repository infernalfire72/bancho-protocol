use crate::serde::byte_sized::ByteSized;
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct Privileges: u32 {
        const NONE = 0;
        const PLAYER = 1;
        const MODERATOR = 2;
        const SUPPORTER = 4;
        const LE_GUY = 8;
        const DEVELOPER = 16;
        const TOURNAMENT_STAFF = 32;
    }
}

impl ByteSized for Privileges {
    fn byte_size(&self) -> usize {
        std::mem::size_of::<u32>()
    }
}

impl BinarySerialize for Privileges {
    fn write_to(&self, writer: &mut BinaryWriter) {
        u32::write_to(&self.bits(), writer)
    }
}
