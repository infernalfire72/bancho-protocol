use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeTeam;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_change_team_deserialize_empty() {
        let data = [];
        let msg = MatchChangeTeam::deserialize(&data).unwrap();
        assert_eq!(std::mem::size_of_val(&msg), 0);
    }
}
