use crate::serde::macros::BinaryDeserialize;
use crate::structures::ScoreFrame;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct UpdateMatchScore {
    pub score: ScoreFrame,
}
