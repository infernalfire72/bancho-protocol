use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use crate::serde::osu_types::PrefixedVec;

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
