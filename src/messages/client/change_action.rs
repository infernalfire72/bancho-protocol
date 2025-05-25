use crate::serde::macros::BinaryDeserialize;
use crate::structures::UserAction;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ChangeAction<'a> {
    pub action: UserAction<'a>,
}
