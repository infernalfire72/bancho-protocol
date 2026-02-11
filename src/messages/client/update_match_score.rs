use crate::serde::macros::BinaryDeserialize;
use crate::structures::ScoreFrame;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct UpdateMatchScore {
    pub score: ScoreFrame,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_update_match_score_deserialize() {
        // ScoreFrame with basic data
        let data = [
            0, 0, 0, 0,  // time
            0,           // id
            0, 0, 0, 0,  // score
            0, 0,        // count300
            0, 0,        // count100
            0, 0,        // count50
            0, 0,        // count_geki
            0, 0,        // count_katu
            0, 0,        // count_miss
            0,           // perfect
            0, 0, 0, 0,  // current_combo
            0, 0, 0, 0,  // max_combo
        ];
        let msg = UpdateMatchScore::deserialize(&data).unwrap();
        assert_eq!(msg.score.time, 0);
    }

    #[test]
    fn test_update_match_score_debug_format() {
        let data = [
            0, 0, 0, 0,  // time
            0,           // id
            0, 0, 0, 0,  // score
            0, 0,        // count300
            0, 0,        // count100
            0, 0,        // count50
            0, 0,        // count_geki
            0, 0,        // count_katu
            0, 0,        // count_miss
            0,           // perfect
            0, 0, 0, 0,  // current_combo
            0, 0, 0, 0,  // max_combo
        ];
        let msg = UpdateMatchScore::deserialize(&data).unwrap();
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("UpdateMatchScore"));
    }
}
