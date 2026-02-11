use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct TournamentMatchInfoRequest {
    pub match_id: i32,
}

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct TournamentJoinMatchChannel {
    pub match_id: i32,
}

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct TournamentLeaveMatchChannel {
    pub match_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_tournament_match_info_request_zero() {
        let data = [0, 0, 0, 0];
        let msg = TournamentMatchInfoRequest::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 0);
    }

    #[test]
    fn test_tournament_match_info_request_valid() {
        let data = [100, 0, 0, 0];
        let msg = TournamentMatchInfoRequest::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 100);
    }

    #[test]
    fn test_tournament_join_match_channel_valid() {
        let data = [50, 0, 0, 0];
        let msg = TournamentJoinMatchChannel::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 50);
    }

    #[test]
    fn test_tournament_leave_match_channel_valid() {
        let data = [75, 0, 0, 0];
        let msg = TournamentLeaveMatchChannel::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 75);
    }

}
