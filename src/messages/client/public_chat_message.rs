use crate::structures::IrcMessage;
use crate::serde::macros::BinaryDeserialize;


// fix tuple structs for deserialize
#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct PublicChatMessage<'a> {
    pub message: IrcMessage<'a>,
}