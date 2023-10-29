use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeHost<'a> {
    pub slot_id: i32,
}