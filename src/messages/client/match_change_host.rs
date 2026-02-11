use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeHost {
    pub slot_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_change_host_slot_zero() {
        let data = [0, 0, 0, 0];
        let msg = MatchChangeHost::deserialize(&data).unwrap();
        assert_eq!(msg.slot_id, 0);
    }

    #[test]
    fn test_match_change_host_valid_slot() {
        let data = [10, 0, 0, 0];
        let msg = MatchChangeHost::deserialize(&data).unwrap();
        assert_eq!(msg.slot_id, 10);
    }

    #[test]
    fn test_match_change_host_debug_format() {
        let msg = MatchChangeHost { slot_id: 5 };
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("5"));
    }
}
