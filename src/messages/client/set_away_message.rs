use crate::structures::IrcMessage;
use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct SetAwayMessage<'a> {
    pub message: IrcMessage<'a>,
}