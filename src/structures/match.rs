use crate::structures::MatchSlot;
use crate::structures::{MatchTeamType, Mode, Mods, WinCondition};
use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[derive(Debug, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized, Clone)]
#[crate_root(crate)]
pub struct Match<'a> {
    pub id: u16,
    pub in_progress: bool,
    pub powerplay: bool,
    pub mods: Mods,

    pub name: &'a str,
    pub password: &'a str,
    pub beatmap_name: &'a str,
    pub beatmap_id: i32,
    pub beatmap_md5: &'a str,
    pub slots: [MatchSlot; 16],

    pub host: i32,
    pub mode: Mode,
    pub win_condition: WinCondition,
    pub team_type: MatchTeamType,

    pub freemod_enabled: bool,
    #[depends(freemod_enabled)]
    pub freemods: Option<[Mods; 16]>,
    pub random_seed: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::{SlotStatus, MatchTeam};
    use crate::serde::{BinarySerialize, BinaryDeserialize};

    #[test]
    fn test_match_empty_no_freemod() {
        let match_obj = Match {
            id: 1,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "test match",
            password: "",
            beatmap_name: "Test Map",
            beatmap_id: 12345,
            beatmap_md5: "abc123",
            slots: [MatchSlot::default(); 16],
            host: 1000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_with_password() {
        let match_obj = Match {
            id: 2,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "private match",
            password: "secret123",
            beatmap_name: "Secret Map",
            beatmap_id: 54321,
            beatmap_md5: "def456",
            slots: [MatchSlot::default(); 16],
            host: 2000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_in_progress() {
        let match_obj = Match {
            id: 3,
            in_progress: true,
            powerplay: false,
            mods: Mods::None,
            name: "ongoing match",
            password: "",
            beatmap_name: "Current Map",
            beatmap_id: 99999,
            beatmap_md5: "xyz789",
            slots: [MatchSlot::default(); 16],
            host: 3000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_powerplay() {
        let match_obj = Match {
            id: 4,
            in_progress: false,
            powerplay: true,
            mods: Mods::None,
            name: "powerplay match",
            password: "",
            beatmap_name: "Power Map",
            beatmap_id: 55555,
            beatmap_md5: "pp1234",
            slots: [MatchSlot::default(); 16],
            host: 4000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_with_mods() {
        let match_obj = Match {
            id: 5,
            in_progress: false,
            powerplay: false,
            mods: Mods::Hidden | Mods::Doubletime,
            name: "modded match",
            password: "",
            beatmap_name: "Modded Map",
            beatmap_id: 77777,
            beatmap_md5: "mod5678",
            slots: [MatchSlot::default(); 16],
            host: 5000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_taiko_mode() {
        let match_obj = Match {
            id: 6,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "taiko match",
            password: "",
            beatmap_name: "Taiko Map",
            beatmap_id: 88888,
            beatmap_md5: "taiko123",
            slots: [MatchSlot::default(); 16],
            host: 6000,
            mode: Mode::Taiko,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_freemod_enabled_without_mods() {
        let freemods = [Mods::None; 16];
        let match_obj = Match {
            id: 13,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "freemod match",
            password: "",
            beatmap_name: "Freemod Map",
            beatmap_id: 55556,
            beatmap_md5: "free555",
            slots: [MatchSlot::default(); 16],
            host: 13000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: true,
            freemods: Some(freemods),
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_freemod_with_different_mods() {
        let mut freemods = [Mods::None; 16];
        freemods[0] = Mods::Hidden;
        freemods[1] = Mods::Doubletime;
        freemods[2] = Mods::HardRock | Mods::Perfect;
        let match_obj = Match {
            id: 14,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "varied freemod match",
            password: "",
            beatmap_name: "Varied Freemod Map",
            beatmap_id: 66667,
            beatmap_md5: "varied666",
            slots: [MatchSlot::default(); 16],
            host: 14000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: true,
            freemods: Some(freemods),
            random_seed: 100,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_with_occupied_slots() {
        let mut slots = [MatchSlot::default(); 16];
        slots[0] = MatchSlot {
            status: SlotStatus::Ready,
            team: MatchTeam::None,
            user_id: 1001,
        };
        slots[1] = MatchSlot {
            status: SlotStatus::Playing,
            team: MatchTeam::Red,
            user_id: 1002,
        };
        slots[2] = MatchSlot {
            status: SlotStatus::NotReady,
            team: MatchTeam::Blue,
            user_id: 1003,
        };
        let match_obj = Match {
            id: 15,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "occupied slots match",
            password: "",
            beatmap_name: "Slots Map",
            beatmap_id: 77778,
            beatmap_md5: "slots777",
            slots,
            host: 15000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 42,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_unicode_names() {
        let match_obj = Match {
            id: 16,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "日本語マッチ",
            password: "",
            beatmap_name: "日本語マップ",
            beatmap_id: 88889,
            beatmap_md5: "unicode888",
            slots: [MatchSlot::default(); 16],
            host: 16000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert_eq!(match_obj, deserialized);
    }

    #[test]
    fn test_match_freemod_false_should_not_serialize_mods() {
        let match_obj = Match {
            id: 19,
            in_progress: false,
            powerplay: false,
            mods: Mods::None,
            name: "no freemod match",
            password: "",
            beatmap_name: "No Freemod Map",
            beatmap_id: 22223,
            beatmap_md5: "nofm111",
            slots: [MatchSlot::default(); 16],
            host: 19000,
            mode: Mode::Standard,
            win_condition: WinCondition::Score,
            team_type: MatchTeamType::HeadToHead,
            freemod_enabled: false,
            freemods: None,
            random_seed: 0,
        };
        let serialized = match_obj.serialize();
        let deserialized = Match::deserialize(&serialized).unwrap();
        assert!(!deserialized.freemod_enabled);
        assert_eq!(deserialized.freemods, None);
    }
}
