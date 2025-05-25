use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct LeaveChannel<'a> {
    pub name: &'a str,
}
