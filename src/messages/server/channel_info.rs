use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::ChannelInfo)]
pub struct ChannelInfo<'a> {
    pub name: &'a str,
    pub topic: &'a str,
    pub user_count: i16,
}
