use crate::serde::macros::BinaryDeserialize;
use crate::serde::osu_types::PrefixedVec;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct UserStatsRequest<'a> {
    pub user_ids: PrefixedVec<i16, i32>,
}