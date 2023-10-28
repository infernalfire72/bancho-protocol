use crate::messages::{Message, MessageType};
use crate::structures::IrcMessage;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, Clone, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::ChatMessage)]
pub struct ChatMessage<'a>(pub &'a IrcMessage);