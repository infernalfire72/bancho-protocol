use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeSlot {
    pub slot_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_change_slot_zero() {
        let data = [0, 0, 0, 0];
        let msg = MatchChangeSlot::deserialize(&data).unwrap();
        assert_eq!(msg.slot_id, 0);
    }

    #[test]
    fn test_match_change_slot_valid() {
        let data = [7, 0, 0, 0];
        let msg = MatchChangeSlot::deserialize(&data).unwrap();
        assert_eq!(msg.slot_id, 7);
    }

    #[test]
    fn test_match_change_slot_max() {
        let data = [255, 255, 255, 127];
        let msg = MatchChangeSlot::deserialize(&data).unwrap();
        assert_eq!(msg.slot_id, i32::MAX);
    }
}
