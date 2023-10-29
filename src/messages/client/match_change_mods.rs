use crate::serde::macros::BinaryDeserialize;
use crate::structures::Mod;


#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeMods {
    pub mods: Mod,
}