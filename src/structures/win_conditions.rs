use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum WinCondition {
    Score,
    Accuracy,
    Combo,
    ScoreV2,
}

impl TryFrom<u8> for WinCondition {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(WinCondition::Score),
            1 => Ok(WinCondition::Accuracy),
            2 => Ok(WinCondition::Combo),
            3 => Ok(WinCondition::ScoreV2),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                "invalid win condition",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_condition_score() {
        let wc = WinCondition::Score;
        assert_eq!(wc as u8, 0);
    }

    #[test]
    fn test_win_condition_accuracy() {
        let wc = WinCondition::Accuracy;
        assert_eq!(wc as u8, 1);
    }

    #[test]
    fn test_win_condition_combo() {
        let wc = WinCondition::Combo;
        assert_eq!(wc as u8, 2);
    }

    #[test]
    fn test_win_condition_score_v2() {
        let wc = WinCondition::ScoreV2;
        assert_eq!(wc as u8, 3);
    }

    #[test]
    fn test_win_condition_copy_clone() {
        let wc1 = WinCondition::Accuracy;
        let wc2 = wc1;
        assert_eq!(wc1, wc2);
    }

    #[test]
    fn test_win_condition_equality() {
        assert_eq!(WinCondition::Score, WinCondition::Score);
        assert_ne!(WinCondition::Score, WinCondition::Accuracy);
        assert_ne!(WinCondition::Combo, WinCondition::ScoreV2);
    }

    #[test]
    fn test_win_condition_try_from_score() {
        assert_eq!(WinCondition::try_from(0).unwrap(), WinCondition::Score);
    }

    #[test]
    fn test_win_condition_try_from_accuracy() {
        assert_eq!(WinCondition::try_from(1).unwrap(), WinCondition::Accuracy);
    }

    #[test]
    fn test_win_condition_try_from_combo() {
        assert_eq!(WinCondition::try_from(2).unwrap(), WinCondition::Combo);
    }

    #[test]
    fn test_win_condition_try_from_score_v2() {
        assert_eq!(WinCondition::try_from(3).unwrap(), WinCondition::ScoreV2);
    }

    #[test]
    fn test_win_condition_try_from_invalid() {
        assert!(WinCondition::try_from(4).is_err());
        assert!(WinCondition::try_from(255).is_err());
    }

    #[test]
    fn test_win_condition_debug_format() {
        let wc = WinCondition::Accuracy;
        let debug_str = format!("{:?}", wc);
        assert_eq!(debug_str, "Accuracy");
    }

    #[test]
    fn test_win_condition_all_variants() {
        let conditions = [
            WinCondition::Score,
            WinCondition::Accuracy,
            WinCondition::Combo,
            WinCondition::ScoreV2,
        ];
        for (i, condition) in conditions.iter().enumerate() {
            assert_eq!(*condition as u8, i as u8);
        }
    }

    // WinCondition serde roundtrip (exercises derive-generated code)
    #[test]
    fn test_win_condition_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        for wc in [WinCondition::Score, WinCondition::Accuracy, WinCondition::Combo, WinCondition::ScoreV2] {
            let bytes = wc.serialize();
            let decoded = WinCondition::deserialize(&bytes).unwrap();
            assert_eq!(wc, decoded);
        }
    }

    #[test]
    fn test_win_condition_deserialize_invalid() {
        use crate::serde::BinaryDeserialize;
        let data = [4u8]; // invalid: max valid is 3 (ScoreV2)
        assert!(WinCondition::deserialize(&data).is_err());
    }

    #[test]
    fn test_win_condition_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(WinCondition::deserialize(&data).is_err());
    }
}
