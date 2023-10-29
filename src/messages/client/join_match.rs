use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct JoinMatch<'a> {
    pub match_id: i32,
    pub password: &'a str,
}