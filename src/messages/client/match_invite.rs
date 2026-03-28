use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchInvite {
    pub user_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_invite_zero_user() {
        let data = [0, 0, 0, 0];
        let msg = MatchInvite::deserialize(&data).unwrap();
        assert_eq!(msg.user_id, 0);
    }

    #[test]
    fn test_match_invite_valid_user() {
        let data = [200, 0, 0, 0];
        let msg = MatchInvite::deserialize(&data).unwrap();
        assert_eq!(msg.user_id, 200);
    }

    #[test]
    fn test_match_invite_negative_user() {
        let data = [255, 255, 255, 255];
        let msg = MatchInvite::deserialize(&data).unwrap();
        assert_eq!(msg.user_id, -1);
    }
}
