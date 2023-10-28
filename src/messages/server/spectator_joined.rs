use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::SpectatorJoined)]
pub struct SpectatorJoined {
    user_id: i32,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::FellowSpectatorJoined)]
pub struct FellowSpectatorJoined {
    user_id: i32,
}