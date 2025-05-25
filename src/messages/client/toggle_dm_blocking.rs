use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ToggleBlockNonFriendDms {
    pub val: i32,
}
