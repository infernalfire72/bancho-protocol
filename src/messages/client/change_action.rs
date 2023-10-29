use crate::structures::UserAction;
use crate::serde::macros::BinaryDeserialize;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ChangeAction<'a> {
    pub action: UserAction<'a>
}