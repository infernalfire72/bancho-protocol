use crate::serde::macros::BinaryDeserialize;
use crate::serde::osu_types::PrefixedVec;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct RequestPresences {
    pub user_ids: PrefixedVec<i16, i32>,
}

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct RequestAllPresences {
    pub ingame_time: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_request_presences_empty() {
        let data = [0, 0];
        let msg = RequestPresences::deserialize(&data).unwrap();
        assert_eq!(msg.user_ids.0.len(), 0);
    }

    #[test]
    fn test_request_presences_with_users() {
        let data = [1, 0, 42, 0, 0, 0];
        let msg = RequestPresences::deserialize(&data).unwrap();
        assert_eq!(msg.user_ids.0.len(), 1);
        assert_eq!(msg.user_ids.0[0], 42);
    }

    #[test]
    fn test_request_all_presences_zero_time() {
        let data = [0, 0, 0, 0];
        let msg = RequestAllPresences::deserialize(&data).unwrap();
        assert_eq!(msg.ingame_time, 0);
    }

    #[test]
    fn test_request_all_presences_positive_time() {
        let data = [200, 0, 0, 0];
        let msg = RequestAllPresences::deserialize(&data).unwrap();
        assert_eq!(msg.ingame_time, 200);
    }

}
