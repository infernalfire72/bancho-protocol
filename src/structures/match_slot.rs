use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use crate::structures::{MatchTeam, SlotStatus};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MatchSlot {
    pub status: SlotStatus,
    pub team: MatchTeam,
    pub user_id: i32,
}

impl MatchSlot {
    pub fn has_user(&self) -> bool {
        self.status != SlotStatus::Empty && self.status != SlotStatus::Locked
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
        self.iter()
            .filter_map(|slot| {
                if slot.user_id == 0 {
                    None
                } else {
                    Some(slot.user_id.byte_size())
                }
            })
            .sum::<usize>()
            + N * 2
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

impl<'a, const N: usize> BinaryDeserialize<'a> for [MatchSlot; N] {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>
    where
        Self: Sized,
    {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::MatchTeam;
    use crate::serde::{BinaryDeserialize, BinarySerialize};

    #[test]
    fn test_match_slot_default() {
        let slot = MatchSlot::default();
        assert_eq!(slot.status, SlotStatus::Empty);
        assert_eq!(slot.team, MatchTeam::None);
        assert_eq!(slot.user_id, 0);
    }

    #[test]
    fn test_match_slot_has_user_empty() {
        let slot = MatchSlot {
            status: SlotStatus::Empty,
            team: MatchTeam::None,
            user_id: 0,
        };
        assert!(!slot.has_user());
    }

    #[test]
    fn test_match_slot_has_user_locked() {
        let slot = MatchSlot {
            status: SlotStatus::Locked,
            team: MatchTeam::None,
            user_id: 0,
        };
        assert!(!slot.has_user());
    }

    #[test]
    fn test_match_slot_has_user_ready() {
        let slot = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: 1000,
        };
        assert!(slot.has_user());
    }

    #[test]
    fn test_match_slot_has_user_not_ready() {
        let slot = MatchSlot {
            status: SlotStatus::NotReady,
            team: MatchTeam::None,
            user_id: 2000,
        };
        assert!(slot.has_user());
    }

    #[test]
    fn test_match_slot_has_user_playing() {
        let slot = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: 3000,
        };
        assert!(slot.has_user());
    }

    #[test]
    fn test_match_slot_has_user_quit() {
        let slot = MatchSlot {
            status: SlotStatus::Quit,
            team: MatchTeam::Blue,
            user_id: 4000,
        };
        assert!(slot.has_user());
    }

    #[test]
    fn test_match_slot_missing_beatmap_has_user() {
        let slot = MatchSlot {
            status: SlotStatus::MissingBeatmap,
            team: MatchTeam::None,
            user_id: 5000,
        };
        assert!(slot.has_user());
    }

    #[test]
    fn test_match_slots_array_default() {
        let slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        for slot in &slots {
            assert!(!slot.has_user());
        }
    }

    #[test]
    fn test_match_slots_array_with_users() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: 1000,
        };
        slots[1] = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: 2000,
        };
        slots[5] = MatchSlot {
            status: SlotStatus::NotReady,
            team: MatchTeam::Blue,
            user_id: 3000,
        };

        assert!(slots[0].has_user());
        assert!(slots[1].has_user());
        assert!(!slots[2].has_user());
        assert!(slots[5].has_user());
    }

    #[test]
    fn test_match_slots_roundtrip_empty() {
        let slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        let serialized = slots.serialize();
        let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

        for i in 0..16 {
            assert_eq!(slots[i].status, deserialized[i].status);
            assert_eq!(slots[i].team, deserialized[i].team);
            assert_eq!(slots[i].user_id, deserialized[i].user_id);
        }
    }

    #[test]
    fn test_match_slots_roundtrip_mixed() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: 1000,
        };
        slots[1] = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: 2000,
        };
        slots[2] = MatchSlot {
            status: SlotStatus::NotReady,
            team: MatchTeam::Blue,
            user_id: 3000,
        };
        slots[3] = MatchSlot {
            status: SlotStatus::Empty,
            team: MatchTeam::None,
            user_id: 0,
        };
        slots[5] = MatchSlot {
            status: SlotStatus::MissingBeatmap,
            team: MatchTeam::Red,
            user_id: 4000,
        };

        let serialized = slots.serialize();
        let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

        for i in 0..16 {
            assert_eq!(slots[i].status, deserialized[i].status);
            assert_eq!(slots[i].team, deserialized[i].team);
            assert_eq!(slots[i].user_id, deserialized[i].user_id);
        }
    }

    #[test]
    fn test_match_slots_only_empty_and_locked() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Empty,
            team: MatchTeam::None,
            user_id: 0,
        };
        slots[5] = MatchSlot {
            status: SlotStatus::Locked,
            team: MatchTeam::None,
            user_id: 0,
        };

        let serialized = slots.serialize();
        let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

        for i in 0..16 {
            assert_eq!(slots[i].status, deserialized[i].status);
            assert_eq!(slots[i].team, deserialized[i].team);
            assert_eq!(slots[i].user_id, deserialized[i].user_id);
        }
    }

    #[test]
    fn test_match_slots_all_filled() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        for i in 0..16 {
            slots[i] = MatchSlot {
                status: SlotStatus::Ready,
                team: if i % 2 == 0 { MatchTeam::Red } else { MatchTeam::Blue },
                user_id: 1000 + i as i32,
            };
        }

        let serialized = slots.serialize();
        let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

        for i in 0..16 {
            assert_eq!(slots[i].status, deserialized[i].status);
            assert_eq!(slots[i].team, deserialized[i].team);
            assert_eq!(slots[i].user_id, deserialized[i].user_id);
        }
    }

    #[test]
    fn test_match_slots_large_user_ids() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: i32::MAX,
        };
        slots[1] = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: i32::MIN,
        };
        slots[2] = MatchSlot {
            status: SlotStatus::NotReady,
            team: MatchTeam::Blue,
            user_id: -1,
        };

        let serialized = slots.serialize();
        let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

        assert_eq!(deserialized[0].user_id, i32::MAX);
        assert_eq!(deserialized[1].user_id, i32::MIN);
        assert_eq!(deserialized[2].user_id, -1);
    }

    #[test]
    fn test_match_slots_byte_size_empty() {
        let slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        let byte_size = slots.byte_size();
        let serialized = slots.serialize();
        assert_eq!(byte_size, serialized.len());
    }

    #[test]
    fn test_match_slots_byte_size_with_users() {
        let mut slots: [MatchSlot; 16] = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: 1000,
        };
        slots[5] = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: 2000,
        };

        let byte_size = slots.byte_size();
        let serialized = slots.serialize();
        assert_eq!(byte_size, serialized.len());
    }

    #[test]
    fn test_match_slot_copy_clone() {
        let slot1 = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::Red,
            user_id: 1000,
        };
        let slot2 = slot1;

        assert_eq!(slot1.status, slot2.status);
        assert_eq!(slot1.team, slot2.team);
        assert_eq!(slot1.user_id, slot2.user_id);
    }
}
