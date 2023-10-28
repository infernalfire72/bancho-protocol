use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::SpectatorLeft)]
pub struct SpectatorLeft {
    user_id: i32,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::FellowSpectatorLeft)]
pub struct FellowSpectatorLeft {
    user_id: i32,
}