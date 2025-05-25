use crate::messages::MessageType;
use crate::structures::Match;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchStart)]
pub struct MatchStart<'a>(pub &'a Match<'a>);
