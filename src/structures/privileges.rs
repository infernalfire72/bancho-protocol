use std::ops::{BitAnd, BitOr, BitXor};
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

impl BitAnd for Privilege {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) & (rhs as u32)) }
    }
}

impl BitOr for Privilege {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) | (rhs as u32)) }
    }
}

impl BitXor for Privilege {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) ^ (rhs as u32)) }
    }
}