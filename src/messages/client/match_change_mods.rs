use crate::serde::macros::BinaryDeserialize;
use crate::structures::Mods;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeMods {
    pub mods: Mods,
}
