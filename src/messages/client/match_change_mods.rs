use crate::serde::macros::BinaryDeserialize;
use crate::structures::Mods;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeMods {
    pub mods: Mods,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_match_change_mods_none() {
        let data = [0, 0, 0, 0];
        let msg = MatchChangeMods::deserialize(&data).unwrap();
        assert_eq!(msg.mods.bits(), 0);
    }

    #[test]
    fn test_match_change_mods_with_mods() {
        // Mods with value 1 (NoFail)
        let data = [1, 0, 0, 0];
        let msg = MatchChangeMods::deserialize(&data).unwrap();
        assert_eq!(msg.mods.bits(), 1);
    }

    #[test]
    fn test_match_change_mods_multiple() {
        // Mods with multiple bits set
        let data = [5, 0, 0, 0];
        let msg = MatchChangeMods::deserialize(&data).unwrap();
        assert_eq!(msg.mods.bits(), 5);
    }

    #[test]
    fn test_match_change_mods_debug_format() {
        let data = [0, 0, 0, 0];
        let msg = MatchChangeMods::deserialize(&data).unwrap();
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("MatchChangeMods"));
    }
}
