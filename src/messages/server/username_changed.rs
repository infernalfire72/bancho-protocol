use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use bancho_protocol_macros::BinaryDeserialize;

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UsernameChanged)]
pub struct UsernameChanged {
    username_change: String,
}

impl UsernameChanged {
    pub fn new(old_username: &str, new_username: &str) -> Self {
        UsernameChanged {
            username_change: format!("{}>>>>{}", old_username, new_username),
        }
    }
}
