use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ReceiveUpdates<'a> {
    pub filter: u32,
}