use crate::messages::{Message, MessageType};
use crate::structures::ScoreFrame;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchScoreUpdate)]
pub struct MatchScoreUpdate<'a>(pub &'a ScoreFrame);