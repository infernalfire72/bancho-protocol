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
        size_of::<u8>()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_status_none() {
        let status = SlotStatus::None;
        assert_eq!(status.bits(), 0);
    }

    #[test]
    fn test_slot_status_empty() {
        let status = SlotStatus::Empty;
        assert_eq!(status.bits(), 1);
    }

    #[test]
    fn test_slot_status_locked() {
        let status = SlotStatus::Locked;
        assert_eq!(status.bits(), 2);
    }

    #[test]
    fn test_slot_status_not_ready() {
        let status = SlotStatus::NotReady;
        assert_eq!(status.bits(), 4);
    }

    #[test]
    fn test_slot_status_ready() {
        let status = SlotStatus::Ready;
        assert_eq!(status.bits(), 8);
    }

    #[test]
    fn test_slot_status_missing_beatmap() {
        let status = SlotStatus::MissingBeatmap;
        assert_eq!(status.bits(), 16);
    }

    #[test]
    fn test_slot_status_playing() {
        let status = SlotStatus::Playing;
        assert_eq!(status.bits(), 32);
    }

    #[test]
    fn test_slot_status_quit() {
        let status = SlotStatus::Quit;
        assert_eq!(status.bits(), 128);
    }

    #[test]
    fn test_slot_status_combination() {
        let status = SlotStatus::Ready | SlotStatus::Playing;
        assert!(status.contains(SlotStatus::Ready));
        assert!(status.contains(SlotStatus::Playing));
        assert!(!status.contains(SlotStatus::Locked));
    }

    #[test]
    fn test_slot_status_copy_clone() {
        let s1 = SlotStatus::Ready | SlotStatus::MissingBeatmap;
        let s2 = s1;
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_slot_status_equality() {
        assert_eq!(SlotStatus::Empty, SlotStatus::Empty);
        assert_ne!(SlotStatus::Empty, SlotStatus::Locked);
    }

    #[test]
    fn test_slot_status_from_bits_retain() {
        let status = SlotStatus::from_bits_retain(5);
        assert_eq!(status.bits(), 5);
        assert!(status.contains(SlotStatus::Empty));
        assert!(status.contains(SlotStatus::NotReady));
    }

    #[test]
    fn test_slot_status_all_flags() {
        let all = SlotStatus::Empty
            | SlotStatus::Locked
            | SlotStatus::NotReady
            | SlotStatus::Ready
            | SlotStatus::MissingBeatmap
            | SlotStatus::Playing
            | SlotStatus::Quit;
        assert!(all.contains(SlotStatus::Empty));
        assert!(all.contains(SlotStatus::Locked));
        assert!(all.contains(SlotStatus::NotReady));
        assert!(all.contains(SlotStatus::Ready));
        assert!(all.contains(SlotStatus::MissingBeatmap));
        assert!(all.contains(SlotStatus::Playing));
        assert!(all.contains(SlotStatus::Quit));
    }

    #[test]
    fn test_slot_status_intersects() {
        let status1 = SlotStatus::Ready | SlotStatus::Playing;
        let status2 = SlotStatus::Playing | SlotStatus::Locked;
        assert!(status1.intersects(status2));
        assert!(status2.intersects(status1));
    }

    #[test]
    fn test_slot_status_not_intersects() {
        let status1 = SlotStatus::Ready;
        let status2 = SlotStatus::Locked;
        assert!(!status1.intersects(status2));
    }

    // Serde roundtrip tests
    #[test]
    fn test_slot_status_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let statuses = [
            SlotStatus::None,
            SlotStatus::Empty,
            SlotStatus::Locked,
            SlotStatus::NotReady,
            SlotStatus::Ready,
            SlotStatus::MissingBeatmap,
            SlotStatus::Playing,
            SlotStatus::Quit,
        ];
        for status in statuses {
            let bytes = status.serialize();
            let decoded = SlotStatus::deserialize(&bytes).unwrap();
            assert_eq!(status, decoded);
        }
    }

    #[test]
    fn test_slot_status_byte_size() {
        use crate::serde::byte_sized::ByteSized;
        assert_eq!(SlotStatus::Ready.byte_size(), 1);
        assert_eq!((SlotStatus::Ready | SlotStatus::Playing).byte_size(), 1);
    }

    #[test]
    fn test_slot_status_serde_combined_flags() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let status = SlotStatus::Ready | SlotStatus::Playing;
        let bytes = status.serialize();
        let decoded = SlotStatus::deserialize(&bytes).unwrap();
        assert_eq!(status, decoded);
    }
}
