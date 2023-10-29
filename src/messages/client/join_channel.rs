use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct JoinChannel<'a> {
    pub name: &'a str,
}