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