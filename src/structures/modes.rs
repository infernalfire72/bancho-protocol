use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use std::io::Error;

#[repr(u8)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized,
)]
#[crate_root(crate)]
pub enum Mode {
    #[default]
    Standard,
    Taiko,
    Catch,
    Mania,
}

impl Mode {
    pub fn from_np(np_mode: &str) -> Option<Mode> {
        match np_mode {
            "Taiko" => Some(Mode::Taiko),
            "CatchTheBeat" => Some(Mode::Catch),
            "osu!mania" => Some(Mode::Mania),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(mut value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            value = 0;
        }

        // SAFETY: `Mode` has 4 variants
        Ok(unsafe { std::mem::transmute(value) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_standard_default() {
        assert_eq!(Mode::default(), Mode::Standard);
    }

    #[test]
    fn test_mode_standard_value() {
        let m = Mode::Standard;
        assert_eq!(m as u8, 0);
    }

    #[test]
    fn test_mode_taiko_value() {
        let m = Mode::Taiko;
        assert_eq!(m as u8, 1);
    }

    #[test]
    fn test_mode_catch_value() {
        let m = Mode::Catch;
        assert_eq!(m as u8, 2);
    }

    #[test]
    fn test_mode_mania_value() {
        let m = Mode::Mania;
        assert_eq!(m as u8, 3);
    }

    #[test]
    fn test_mode_copy_clone() {
        let m1 = Mode::Catch;
        let m2 = m1;
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_mode_equality() {
        assert_eq!(Mode::Standard, Mode::Standard);
        assert_ne!(Mode::Standard, Mode::Taiko);
        assert_ne!(Mode::Taiko, Mode::Catch);
        assert_ne!(Mode::Catch, Mode::Mania);
    }

    #[test]
    fn test_mode_try_from_valid() {
        assert_eq!(Mode::try_from(0).unwrap(), Mode::Standard);
        assert_eq!(Mode::try_from(1).unwrap(), Mode::Taiko);
        assert_eq!(Mode::try_from(2).unwrap(), Mode::Catch);
        assert_eq!(Mode::try_from(3).unwrap(), Mode::Mania);
    }

    #[test]
    fn test_mode_try_from_out_of_range() {
        assert_eq!(Mode::try_from(4).unwrap(), Mode::Standard);
        assert_eq!(Mode::try_from(5).unwrap(), Mode::Standard);
        assert_eq!(Mode::try_from(255).unwrap(), Mode::Standard);
    }

    #[test]
    fn test_mode_from_np_taiko() {
        assert_eq!(Mode::from_np("Taiko"), Some(Mode::Taiko));
    }

    #[test]
    fn test_mode_from_np_catch() {
        assert_eq!(Mode::from_np("CatchTheBeat"), Some(Mode::Catch));
    }

    #[test]
    fn test_mode_from_np_mania() {
        assert_eq!(Mode::from_np("osu!mania"), Some(Mode::Mania));
    }

    #[test]
    fn test_mode_from_np_standard_none() {
        assert_eq!(Mode::from_np("osu!"), None);
    }

    #[test]
    fn test_mode_from_np_invalid() {
        assert_eq!(Mode::from_np("InvalidMode"), None);
        assert_eq!(Mode::from_np(""), None);
    }

    #[test]
    fn test_mode_debug_format() {
        let m = Mode::Mania;
        let debug_str = format!("{:?}", m);
        assert_eq!(debug_str, "Mania");
    }

    #[test]
    fn test_mode_all_variants() {
        let modes = [Mode::Standard, Mode::Taiko, Mode::Catch, Mode::Mania];
        for (i, mode) in modes.iter().enumerate() {
            assert_eq!(*mode as u8, i as u8);
        }
    }

    // Mode serde roundtrip (exercises derive-generated code)
    #[test]
    fn test_mode_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        for mode in [Mode::Standard, Mode::Taiko, Mode::Catch, Mode::Mania] {
            let bytes = mode.serialize();
            let decoded = Mode::deserialize(&bytes).unwrap();
            assert_eq!(mode, decoded);
        }
    }

    #[test]
    fn test_mode_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(Mode::deserialize(&data).is_err());
    }
}
