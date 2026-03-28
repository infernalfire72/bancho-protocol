use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[derive(Debug, Default, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct ScoreFrame {
    pub time: i32,
    pub slot_id: u8,
    pub num300: u16,
    pub num100: u16,
    pub num50: u16,
    pub num_geki: u16,
    pub num_katu: u16,
    pub misses: u16,
    pub total_score: i32,
    pub current_combo: u16,
    pub max_combo: u16,
    pub perfect: bool,
    pub current_hp: u8,
    pub tag_byte: u8,
    pub score_v2: bool,

    #[depends(score_v2)]
    pub combo_portion: Option<f64>,
    #[depends(score_v2)]
    pub bonus_portion: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::{BinarySerialize, BinaryDeserialize};

    #[test]
    fn test_score_frame_default() {
        let frame = ScoreFrame::default();
        assert_eq!(frame.time, 0);
        assert_eq!(frame.slot_id, 0);
        assert_eq!(frame.num300, 0);
        assert_eq!(frame.total_score, 0);
        assert!(!frame.score_v2);
        assert_eq!(frame.combo_portion, None);
        assert_eq!(frame.bonus_portion, None);
    }

    #[test]
    fn test_score_frame_default_roundtrip() {
        let frame = ScoreFrame::default();
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_no_score_v2() {
        let frame = ScoreFrame {
            time: 1000,
            slot_id: 1,
            num300: 100,
            num100: 50,
            num50: 10,
            num_geki: 5,
            num_katu: 3,
            misses: 1,
            total_score: 50000,
            current_combo: 200,
            max_combo: 300,
            perfect: false,
            current_hp: 200,
            tag_byte: 0,
            score_v2: false,
            combo_portion: None,
            bonus_portion: None,
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_with_score_v2() {
        let frame = ScoreFrame {
            time: 2000,
            slot_id: 2,
            num300: 200,
            num100: 100,
            num50: 20,
            num_geki: 10,
            num_katu: 5,
            misses: 2,
            total_score: 100000,
            current_combo: 400,
            max_combo: 500,
            perfect: true,
            current_hp: 220,
            tag_byte: 1,
            score_v2: true,
            combo_portion: Some(0.75),
            bonus_portion: Some(0.25),
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_perfect() {
        let frame = ScoreFrame {
            time: 0,
            slot_id: 0,
            num300: 50,
            num100: 0,
            num50: 0,
            num_geki: 0,
            num_katu: 0,
            misses: 0,
            total_score: 25000,
            current_combo: 50,
            max_combo: 50,
            perfect: true,
            current_hp: 200,
            tag_byte: 0,
            score_v2: false,
            combo_portion: None,
            bonus_portion: None,
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_failed() {
        let frame = ScoreFrame {
            time: 5000,
            slot_id: 3,
            num300: 30,
            num100: 20,
            num50: 5,
            num_geki: 0,
            num_katu: 0,
            misses: 100,
            total_score: 5000,
            current_combo: 10,
            max_combo: 50,
            perfect: false,
            current_hp: 0,
            tag_byte: 0,
            score_v2: false,
            combo_portion: None,
            bonus_portion: None,
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_max_values() {
        let frame = ScoreFrame {
            time: i32::MAX,
            slot_id: u8::MAX,
            num300: u16::MAX,
            num100: u16::MAX,
            num50: u16::MAX,
            num_geki: u16::MAX,
            num_katu: u16::MAX,
            misses: u16::MAX,
            total_score: i32::MAX,
            current_combo: u16::MAX,
            max_combo: u16::MAX,
            perfect: true,
            current_hp: u8::MAX,
            tag_byte: u8::MAX,
            score_v2: false,
            combo_portion: None,
            bonus_portion: None,
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_negative_time() {
        let frame = ScoreFrame {
            time: -1000,
            slot_id: 5,
            num300: 50,
            num100: 20,
            num50: 5,
            num_geki: 0,
            num_katu: 0,
            misses: 0,
            total_score: 10000,
            current_combo: 75,
            max_combo: 100,
            perfect: false,
            current_hp: 200,
            tag_byte: 0,
            score_v2: false,
            combo_portion: None,
            bonus_portion: None,
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_score_v2_both_portions() {
        let frame = ScoreFrame {
            time: 4500,
            slot_id: 4,
            num300: 150,
            num100: 50,
            num50: 10,
            num_geki: 5,
            num_katu: 2,
            misses: 1,
            total_score: 75000,
            current_combo: 300,
            max_combo: 400,
            perfect: false,
            current_hp: 210,
            tag_byte: 2,
            score_v2: true,
            combo_portion: Some(0.8),
            bonus_portion: Some(0.2),
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_score_frame_conditional_fields_serialized_when_true() {
        let frame = ScoreFrame {
            time: 2000,
            slot_id: 2,
            num300: 100,
            num100: 50,
            num50: 10,
            num_geki: 5,
            num_katu: 3,
            misses: 1,
            total_score: 50000,
            current_combo: 200,
            max_combo: 300,
            perfect: false,
            current_hp: 200,
            tag_byte: 1,
            score_v2: true,
            combo_portion: Some(0.5),
            bonus_portion: Some(0.5),
        };
        let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.combo_portion, Some(0.5));
        assert_eq!(deserialized.bonus_portion, Some(0.5));
    }

    #[test]
    fn test_score_frame_different_slots() {
        for slot_id in 0..=15u8 {
            let frame = ScoreFrame {
                time: 1000,
                slot_id,
                num300: 100,
                num100: 50,
                num50: 10,
                num_geki: 5,
                num_katu: 3,
                misses: 1,
                total_score: 50000,
                current_combo: 200,
                max_combo: 300,
                perfect: false,
                current_hp: 200,
                tag_byte: 0,
                score_v2: false,
                combo_portion: None,
                bonus_portion: None,
            };
            let serialized = frame.serialize();
        let deserialized = ScoreFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
        }
    }
}
