use crate::serde::macros::{BinarySerialize, ByteSized};

#[repr(u32)]
#[derive(Debug, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum Privilege {
    None,
    Player = 1,
    Moderator = 2,
    Supporter = 4,
    LeGuy = 8,
    Developer = 16,
    TournamentStaff = 32,
}