use crate::messages::{Message, MessageType};
use crate::serde::osu_types::PrefixedVec;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::FriendsList)]
pub struct FriendsList {
    friends: PrefixedVec<i16, i32>,
}

impl From<Vec<i32>> for FriendsList {
    fn from(value: Vec<i32>) -> Self {
        Self {
            friends: PrefixedVec::from(value),
        }
    }
}