use crate::structures::ScoreFrame;
use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct UpdateMatchScore {
    pub score: ScoreFrame,
}