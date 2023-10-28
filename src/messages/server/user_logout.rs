use crate::messages::{Message, MessageType};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserLogout)]
pub struct UserLogout {
    user_id: i64, // this is usually user_id: i32, state: u8
}

impl UserLogout {
    pub fn new(user_id: i32) -> Self {
        Self { user_id: user_id as _ }
    }
}