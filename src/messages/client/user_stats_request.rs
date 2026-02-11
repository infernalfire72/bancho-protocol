use crate::serde::macros::BinaryDeserialize;
use crate::serde::osu_types::PrefixedVec;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct UserStatsRequest {
    pub user_ids: PrefixedVec<i16, i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_user_stats_request_empty() {
        // Empty vector: count = 0
        let data = [0, 0];
        let msg = UserStatsRequest::deserialize(&data).unwrap();
        assert_eq!(msg.user_ids.0.len(), 0);
    }

    #[test]
    fn test_user_stats_request_single_user() {
        // One user: count = 1, user_id
        let data = [1, 0, 42, 0, 0, 0];
        let msg = UserStatsRequest::deserialize(&data).unwrap();
        assert_eq!(msg.user_ids.0.len(), 1);
        assert_eq!(msg.user_ids.0[0], 42);
    }
}
