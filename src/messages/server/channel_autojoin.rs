use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::ChannelAutoJoin)]
pub struct ChannelAutoJoin<'a> {
    pub name: &'a str,
    pub topic: &'a str,
    pub user_count: i16,
}