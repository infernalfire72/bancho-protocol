use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use bitflags::bitflags;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

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

pub const MODS_SHORT: [&'static str; 31] = [
    "NF", "EZ", "TD", "HD", "HR", "SD", "DT", "RX", "HT", "NC", "FL", "AT", "SO", "AP", "PF", "4K",
    "5K", "6K", "7K", "8K", "FI", "RN", "CN", "TP", "9K", "CO", "1K", "3K", "2K", "V2", "MR",
];

pub const NP_MODS: [&str; 31] = [
    "-NoFail",
    "-Easy",
    "~TouchDevice~",
    "+Hidden",
    "+HardRock",
    "+SuddenDeath",
    "+DoubleTime",
    "~Relax~",
    "-HalfTime",
    "+Nightcore",
    "+Flashlight",
    "|Autoplay|",
    "-SpunOut",
    "~Autopilot~",
    "+Perfect",
    "|4K|",
    "|5K|",
    "|6K|",
    "|7K|",
    "|8K|",
    "+FadeIn",
    "~Random~",
    "|Cinema|",
    "~Target~",
    "|9K|",
    "|Coop|",
    "|1K|",
    "|3K|",
    "|2K|",
    "~ScoreV2~",
    "~Mirror~",
];

impl Mods {
    pub fn from_code<T: AsRef<[u8]>>(code: T) -> Mods {
        MODS_SHORT
            .into_iter()
            .position(|x| x.as_bytes() == code.as_ref())
            .map(|pos| Mods::from_bits_truncate(1 << pos))
            .unwrap_or(Mods::None)
    }

    pub fn from_np(np_mod: &str) -> Mods {
        match NP_MODS.into_iter().position(|mod_str| mod_str == np_mod) {
            None => Mods::None,
            Some(mod_index) => Mods::from_bits_truncate(1 << mod_index),
        }
    }
}

impl FromStr for Mods {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid mod input"));
        }

        Ok(s.as_bytes()
            .chunks(2)
            .fold(Mods::None, |mods, mod_str| mods | Mods::from_code(mod_str)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mods_default_none() {
        assert_eq!(Mods::default(), Mods::None);
        assert_eq!(Mods::default().bits(), 0);
    }

    #[test]
    fn test_mods_individual_flags() {
        assert_eq!(Mods::NoFail.bits(), 1 << 0);
        assert_eq!(Mods::Easy.bits(), 1 << 1);
        assert_eq!(Mods::Hidden.bits(), 1 << 3);
        assert_eq!(Mods::HardRock.bits(), 1 << 4);
        assert_eq!(Mods::Doubletime.bits(), 1 << 6);
        assert_eq!(Mods::Flashlight.bits(), 1 << 10);
    }

    #[test]
    fn test_mods_combination() {
        let mods = Mods::Hidden | Mods::HardRock;
        assert!(mods.contains(Mods::Hidden));
        assert!(mods.contains(Mods::HardRock));
        assert!(!mods.contains(Mods::Easy));
    }

    #[test]
    fn test_mods_has_all() {
        let mods = Mods::Hidden | Mods::HardRock;
        assert!(mods.has_all(Mods::Hidden));
        assert!(mods.has_all(Mods::HardRock));
        assert!(mods.has_all(Mods::Hidden | Mods::HardRock));
        assert!(!mods.has_all(Mods::Easy));
    }

    #[test]
    fn test_mods_has_any() {
        let mods = Mods::Hidden | Mods::HardRock;
        assert!(mods.has_any(Mods::Hidden));
        assert!(mods.has_any(Mods::HardRock));
        assert!(mods.has_any(Mods::Hidden | Mods::Easy));
        assert!(!mods.has_any(Mods::Easy));
    }

    #[test]
    fn test_mods_none() {
        assert_eq!(Mods::None.bits(), 0);
        assert!(!Mods::None.contains(Mods::Hidden));
    }

    #[test]
    fn test_mods_from_code_valid() {
        assert_eq!(Mods::from_code("NF"), Mods::NoFail);
        assert_eq!(Mods::from_code("EZ"), Mods::Easy);
        assert_eq!(Mods::from_code("HD"), Mods::Hidden);
        assert_eq!(Mods::from_code("HR"), Mods::HardRock);
        assert_eq!(Mods::from_code("DT"), Mods::Doubletime);
        assert_eq!(Mods::from_code("FL"), Mods::Flashlight);
    }

    #[test]
    fn test_mods_from_code_invalid() {
        assert_eq!(Mods::from_code("XX"), Mods::None);
        assert_eq!(Mods::from_code(""), Mods::None);
    }

    #[test]
    fn test_mods_from_np_valid() {
        assert_eq!(Mods::from_np("+Hidden"), Mods::Hidden);
        assert_eq!(Mods::from_np("+HardRock"), Mods::HardRock);
        assert_eq!(Mods::from_np("+DoubleTime"), Mods::Doubletime);
        assert_eq!(Mods::from_np("-NoFail"), Mods::NoFail);
    }

    #[test]
    fn test_mods_from_np_invalid() {
        assert_eq!(Mods::from_np("InvalidMod"), Mods::None);
        assert_eq!(Mods::from_np(""), Mods::None);
    }

    #[test]
    fn test_mods_from_str_single() {
        let mods = "HD".parse::<Mods>().unwrap();
        assert_eq!(mods, Mods::Hidden);
    }

    #[test]
    fn test_mods_from_str_multiple() {
        let mods = "HDHR".parse::<Mods>().unwrap();
        assert!(mods.contains(Mods::Hidden));
        assert!(mods.contains(Mods::HardRock));
    }

    #[test]
    fn test_mods_from_str_none() {
        let mods = "".parse::<Mods>().unwrap();
        assert_eq!(mods, Mods::None);
    }

    #[test]
    fn test_mods_from_str_invalid_non_ascii() {
        let result = "HD🎮".parse::<Mods>();
        assert!(result.is_err());
    }

    #[test]
    fn test_mods_display_single() {
        let mods = Mods::Hidden;
        assert_eq!(mods.to_string(), "+HD");
    }

    #[test]
    fn test_mods_display_multiple() {
        let mods = Mods::Hidden | Mods::HardRock;
        let s = mods.to_string();
        assert!(s.contains("HD"));
        assert!(s.contains("HR"));
        assert!(s.starts_with("+"));
    }

    #[test]
    fn test_mods_display_none() {
        let mods = Mods::None;
        assert_eq!(mods.to_string(), "");
    }

    #[test]
    fn test_mods_debug_single() {
        let mods = Mods::Hidden;
        let debug_str = format!("{:?}", mods);
        assert!(debug_str.contains("Hidden"));
    }

    #[test]
    fn test_mods_debug_multiple() {
        let mods = Mods::Hidden | Mods::HardRock;
        let debug_str = format!("{:?}", mods);
        assert!(debug_str.contains("Hidden"));
        assert!(debug_str.contains("HardRock"));
    }

    #[test]
    fn test_mods_bitwise_operations() {
        let m1 = Mods::Hidden | Mods::HardRock;
        let m2 = Mods::HardRock | Mods::Doubletime;

        let union = m1 | m2;
        assert!(union.contains(Mods::Hidden));
        assert!(union.contains(Mods::HardRock));
        assert!(union.contains(Mods::Doubletime));

        let intersection = m1 & m2;
        assert_eq!(intersection, Mods::HardRock);
    }

    #[test]
    fn test_mods_equality() {
        let m1 = Mods::Hidden | Mods::HardRock;
        let m2 = Mods::HardRock | Mods::Hidden;
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_mods_copy_clone() {
        let m1 = Mods::Hidden | Mods::HardRock;
        let m2 = m1;
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_mods_all_key_mods() {
        assert_eq!(Mods::Keys4.bits(), 1 << 15);
        assert_eq!(Mods::Keys5.bits(), 1 << 16);
        assert_eq!(Mods::Keys6.bits(), 1 << 17);
        assert_eq!(Mods::Keys7.bits(), 1 << 18);
        assert_eq!(Mods::Keys8.bits(), 1 << 19);
        assert_eq!(Mods::Keys9.bits(), 1 << 24);
    }

    #[test]
    fn test_mods_speed_mods() {
        assert_eq!(Mods::Doubletime.bits(), 1 << 6);
        assert_eq!(Mods::Halftime.bits(), 1 << 8);
        assert_eq!(Mods::Nightcore.bits(), 1 << 9);
    }

    #[test]
    fn test_mods_difficulty_increase() {
        assert_eq!(Mods::Hidden.bits(), 1 << 3);
        assert_eq!(Mods::HardRock.bits(), 1 << 4);
        assert_eq!(Mods::Flashlight.bits(), 1 << 10);
    }

    #[test]
    fn test_mods_from_bits_truncate() {
        let mods = Mods::from_bits_truncate(0xFF);
        assert!(mods.contains(Mods::NoFail));
        assert!(mods.contains(Mods::Easy));
        assert!(mods.contains(Mods::Hidden));
    }

    #[test]
    fn test_mods_from_code_with_bytes() {
        let code_bytes = b"FL";
        let mods = Mods::from_code(code_bytes);
        assert_eq!(mods, Mods::Flashlight);
    }
}
