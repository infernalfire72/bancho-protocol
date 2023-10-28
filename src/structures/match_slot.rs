use crate::serde::byte_sized::ByteSized;
use crate::serde::ByteStream;
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use crate::serde::deserialize::BinaryDeserialize;
use crate::structures::{MatchTeam, SlotStatus};

#[derive(Debug, Copy, Clone)]
pub struct MatchSlot {
    pub status: SlotStatus,
    pub team: MatchTeam,
    pub user_id: i32,
}

impl MatchSlot {
    pub fn has_user(&self) -> bool {
        self.status != SlotStatus::Empty &&
            self.status != SlotStatus::Locked
    }
}

impl Default for MatchSlot {
    fn default() -> Self {
        Self {
            status: SlotStatus::Empty,
            team: MatchTeam::None,
            user_id: 0,
        }
    }
}

impl<const N: usize> ByteSized for [MatchSlot; N] {
    fn byte_size(&self) -> usize {
        self.iter().filter_map(|slot| if slot.user_id == 0 {
            None
        } else {
            Some(slot.user_id.byte_size())
        }).sum::<usize>() + N * 2
    }
}

impl<const N: usize> BinarySerialize for [MatchSlot; N] {
    fn write_to(&self, writer: &mut BinaryWriter) {
        for i in 0..N {
            self[i].status.write_to(writer);
        }

        for i in 0..N {
            self[i].team.write_to(writer);
        }

        for i in 0..N {
            if self[i].user_id != 0 {
                i32::write_to(&self[i].user_id, writer);
            }
        }
    }
}

impl<const N: usize> BinaryDeserialize for [MatchSlot; N] {
    fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> where Self: Sized {
        let mut slots = [MatchSlot::default(); N];
        for i in 0..N {
            slots[i].status = SlotStatus::read_from(reader)?;
        }

        for i in 0..N {
            slots[i].team = MatchTeam::read_from(reader)?;
        }

        for i in 0..N {
            if slots[i].has_user() {
                slots[i].user_id = i32::read_from(reader)?;
            }
        }

        Ok(slots)
    }
}