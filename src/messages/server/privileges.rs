use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use crate::structures::Privileges;

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::Privileges)]
pub struct UserPrivileges {
    pub privileges: Privileges,
}
