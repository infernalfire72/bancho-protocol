use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchReady;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchNotReady;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchNoBeatmap;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchHasBeatmap;
