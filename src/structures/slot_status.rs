use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct SlotStatus: u8 {
        const None = 0;
        const Empty = 1;
        const Locked = 2;
        const NotReady = 4;
        const Ready = 8;
        const MissingBeatmap = 16;
        const Playing = 32;
        const Quit = 128;
    }
}

impl ByteSized for SlotStatus {
    fn byte_size(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

impl BinarySerialize for SlotStatus {
    fn write_to(&self, writer: &mut BinaryWriter) {
        let bits = self.bits();
        u8::write_to(&bits, writer)
    }
}

impl<'a> BinaryDeserialize<'a> for SlotStatus {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let bits = u8::read_from(reader)?;
        Ok(SlotStatus::from_bits_retain(bits))
    }
}
