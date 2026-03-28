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
    fn test_mode_wire_values() {
        use crate::serde::BinarySerialize;
        assert_eq!(Mode::Standard.serialize(), [0]);
        assert_eq!(Mode::Taiko.serialize(), [1]);
        assert_eq!(Mode::Catch.serialize(), [2]);
        assert_eq!(Mode::Mania.serialize(), [3]);
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
