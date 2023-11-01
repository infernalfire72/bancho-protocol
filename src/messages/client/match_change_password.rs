use crate::structures::Match;
use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangePassword<'a> {
    pub match_data: Match<'a>,
}