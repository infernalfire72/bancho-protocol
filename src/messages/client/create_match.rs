use crate::serde::macros::BinaryDeserialize;
use crate::structures::Match;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct CreateMatch<'a> {
    pub match_data: Match<'a>,
}
