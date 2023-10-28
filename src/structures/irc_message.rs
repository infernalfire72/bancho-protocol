use crate::serde::macros::{BinarySerialize, BinaryDeserialize, ByteSized};

#[derive(Debug, Clone, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct IrcMessage {
    pub sender: String,
    pub text: String,
    pub recipient: String,
    pub sender_id: i32,
}