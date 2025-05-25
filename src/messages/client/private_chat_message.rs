use crate::serde::macros::BinaryDeserialize;
use crate::structures::IrcMessage;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct PrivateChatMessage<'a> {
    pub message: IrcMessage<'a>,
}
