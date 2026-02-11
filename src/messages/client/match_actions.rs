use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchReady;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchNotReady;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchNoBeatmap;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchHasBeatmap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_ready_deserialize() {
        let data = [];
        let msg = MatchReady::deserialize(&data).unwrap();
        assert_eq!(std::mem::size_of_val(&msg), 0);
    }

    #[test]
    fn test_match_not_ready_deserialize() {
        let data = [];
        let msg = MatchNotReady::deserialize(&data).unwrap();
        assert_eq!(std::mem::size_of_val(&msg), 0);
    }

    #[test]
    fn test_match_no_beatmap_deserialize() {
        let data = [];
        let msg = MatchNoBeatmap::deserialize(&data).unwrap();
        assert_eq!(std::mem::size_of_val(&msg), 0);
    }

    #[test]
    fn test_match_has_beatmap_deserialize() {
        let data = [];
        let msg = MatchHasBeatmap::deserialize(&data).unwrap();
        assert_eq!(std::mem::size_of_val(&msg), 0);
    }

}
