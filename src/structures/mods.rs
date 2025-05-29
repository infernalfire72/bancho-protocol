use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use bitflags::bitflags;
use std::fmt::{Debug, Display, Formatter};

bitflags! {
    #[derive(Default, Copy, Clone, Eq, PartialEq)]
    pub struct Mods: u32 {
        const None = 0;
        const NoFail = 1 << 0;
        const Easy = 1 << 1;
        const TouchDevice = 1 << 2;
        const Hidden = 1 << 3;
        const HardRock = 1 << 4;
        const SuddenDeath = 1 << 5;
        const Doubletime = 1 << 6;
        const Relax = 1 << 7;
        const Halftime = 1 << 8;
        const Nightcore = 1 << 9;
        const Flashlight = 1 << 10;
        const Autoplay = 1 << 11;
        const SpunOut = 1 << 12;
        const Autopilot = 1 << 13;
        const Perfect = 1 << 14;
        const Keys4 = 1 << 15;
        const Keys5 = 1 << 16;
        const Keys6 = 1 << 17;
        const Keys7 = 1 << 18;
        const Keys8 = 1 << 19;
        const FadeIn = 1 << 20;
        const Random = 1 << 21;
        const Cinema = 1 << 22;
        const TargetPractice = 1 << 23;
        const Keys9 = 1 << 24;
        const Coop = 1 << 25;
        const Key1 = 1 << 26;
        const Keys3 = 1 << 27;
        const Keys2 = 1 << 28;
        const ScoreV2 = 1 << 29;
        const Mirror = 1 << 30;
    }
}

impl Mods {
    pub fn has_all(&self, mods: Mods) -> bool {
        self.contains(mods)
    }

    pub fn has_any(&self, mods: Mods) -> bool {
        self.intersects(mods)
    }
}

impl ByteSized for Mods {
    fn byte_size(&self) -> usize {
        std::mem::size_of::<u32>()
    }
}

impl BinarySerialize for Mods {
    fn write_to(&self, writer: &mut BinaryWriter) {
        let bits = self.bits();
        u32::write_to(&bits, writer)
    }
}

impl<'a> BinaryDeserialize<'a> for Mods {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let bits = u32::read_from(reader)?;
        Ok(Mods::from_bits_retain(bits))
    }
}

const MODS_DEBUG: [&'static str; 31] = [
    "NoFail",
    "Easy",
    "TouchDevice",
    "Hidden",
    "HardRock",
    "SuddenDeath",
    "Doubletime",
    "Relax",
    "Halftime",
    "Nightcore",
    "Flashlight",
    "Autoplay",
    "SpunOut",
    "Autopilot",
    "Perfect",
    "Keys4",
    "Keys5",
    "Keys6",
    "Keys7",
    "Keys8",
    "FadeIn",
    "Random",
    "Cinema",
    "TargetPractice",
    "Keys9",
    "Coop",
    "Key1",
    "Keys3",
    "Keys2",
    "ScoreV2",
    "Mirror",
];

const MODS_SHORT: [&'static str; 31] = [
    "NF", "EZ", "TD", "HD", "HR", "SD", "DT", "RX", "HT", "NC", "FL", "AT", "SO", "AP", "PF", "4K",
    "5K", "6K", "7K", "8K", "FI", "RN", "CN", "TP", "9K", "CO", "1K", "3K", "2K", "V2", "MR",
];

impl Debug for Mods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut mods = vec![];

        let v = self.bits();
        for i in 0..MODS_DEBUG.len() {
            let debug_str = MODS_DEBUG[i];
            if (v & (1 << i)) != 0 {
                mods.push(debug_str);
            }
        }

        Debug::fmt(&mods, f)
    }
}

impl Display for Mods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut mods = vec![];

        let v = self.bits();
        for i in 0..MODS_SHORT.len() {
            let short_str = MODS_SHORT[i];
            if (v & (1 << i)) != 0 {
                mods.push(short_str);
            }
        }

        if mods.is_empty() {
            return Ok(());
        }

        // TODO: adjust logic here a bit for DTNC/SDPF cases
        let mods_joined = mods.join("");
        let mods_format = format!("+{mods_joined}");
        Display::fmt(&mods_format, f)
    }
}
