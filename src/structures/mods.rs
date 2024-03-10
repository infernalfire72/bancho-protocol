use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use std::ops::{BitAnd, BitOr, BitXor};

#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub enum Mod {
    #[default]
    None,
    NoFail = 1 << 0,
    Easy = 1 << 1,
    TouchDevice = 1 << 2,
    Hidden = 1 << 3,
    HardRock = 1 << 4,
    SuddenDeath = 1 << 5,
    Doubletime = 1 << 6,
    Relax = 1 << 7,
    Halftime = 1 << 8,
    Nightcore = 1 << 9,
    Flashlight = 1 << 10,
    Autoplay = 1 << 11,
    SpunOut = 1 << 12,
    Autopilot = 1 << 13,
    Perfect = 1 << 14,
    Keys4 = 1 << 15,
    Keys5 = 1 << 16,
    Keys6 = 1 << 17,
    Keys7 = 1 << 18,
    Keys8 = 1 << 19,
    FadeIn = 1 << 20,
    Random = 1 << 21,
    Cinema = 1 << 22,
    TargetPractice = 1 << 23,
    Keys9 = 1 << 24,
    Coop = 1 << 25,
    Key1 = 1 << 26,
    Keys3 = 1 << 27,
    Keys2 = 1 << 28,
    ScoreV2 = 1 << 29,
    Mirror = 1 << 30,
}

impl BitAnd for Mod {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) & (rhs as u32)) }
    }
}

impl BitOr for Mod {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) | (rhs as u32)) }
    }
}

impl BitXor for Mod {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute((self as u32) ^ (rhs as u32)) }
    }
}