use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchPlayerSkipped)]
pub struct MatchPlayerSkipped {
    pub slot_id: u32,
}