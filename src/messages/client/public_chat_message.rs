use crate::serde::macros::BinaryDeserialize;
use crate::structures::IrcMessage;

// fix tuple structs for deserialize
#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct PublicChatMessage<'a> {
    pub message: IrcMessage<'a>,
}
