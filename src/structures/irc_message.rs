use crate::serde::macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[derive(Debug, Clone, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct IrcMessage<'a> {
    pub sender: &'a str,
    pub text: &'a str,
    pub recipient: &'a str,
    pub sender_id: i32,
}
