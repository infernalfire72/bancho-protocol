use crate::serde::macros::BinaryDeserialize;
use crate::serde::osu_types::PrefixedVec;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct RequestPresences {
    pub user_ids: PrefixedVec<i16, i32>,
}

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct RequestAllPresences {
    pub ingame_time: i32,
}