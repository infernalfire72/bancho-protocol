use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use bancho_protocol_macros::BinaryDeserialize;

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::Alert)]
pub struct Alert<'a> {
    pub message: &'a str,
}
