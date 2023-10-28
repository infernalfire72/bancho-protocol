use crate::messages::{Message, MessageType};
use crate::structures::Privilege;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::Privileges)]
pub struct Privileges {
    pub privileges: Privilege,
}