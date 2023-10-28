use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::TargetSilenced)]
pub struct TargetSilenced<'a> {
    placeholder: u8,
    placeholder2: u8,
    target: &'a str,
    placeholder3: i32,
}

impl<'a> TargetSilenced<'a> {
    pub fn new(target: &'a str) -> Self {
        Self {
            target,
            placeholder: 0,
            placeholder2: 0,
            placeholder3: 0,
        }
    }
}