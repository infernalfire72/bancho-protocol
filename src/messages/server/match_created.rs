use crate::messages::{Message, MessageType};
use crate::structures::Match;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchCreated)]
pub struct MatchCreated<'a>(pub &'a Match);