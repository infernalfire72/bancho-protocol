use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresence)]
pub struct UserDmBlocked<'a> {
    placeholder: u8,
    placeholder2: u8,
    target: &'a str,
    placeholder3: i32,
}

impl<'a> UserDmBlocked<'a> {
    pub fn new(target: &'a str) -> Self {
        Self {
            target,
            placeholder: 0,
            placeholder2: 0,
            placeholder3: 0,
        }
    }
}
