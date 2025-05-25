use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::SpectatorJoined)]
pub struct SpectatorJoined {
    pub user_id: i32,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::FellowSpectatorJoined)]
pub struct FellowSpectatorJoined {
    pub user_id: i32,
}
