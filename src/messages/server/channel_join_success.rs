use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::ChannelJoinSuccess)]
pub struct ChannelJoinSuccess<'a> {
    pub name: &'a str,
}
