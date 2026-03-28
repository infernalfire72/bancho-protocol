use crate::serde::macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use crate::structures::{Mode, Mods};

#[repr(u8)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized,
)]
#[crate_root(crate)]
pub enum Action {
    #[default]
    Idle,
    AFK,
    Playing,
    Editing,
    Modding,
    Multiplayer,
    Watching,
    Ranking,
    Testing,
    Submitting,
    Paused,
    Lobby,
    Multiplaying,
    Direct,
}

impl TryFrom<u8> for Action {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        match value {
            0 => Ok(Action::Idle),
            1 => Ok(Action::AFK),
            2 => Ok(Action::Playing),
            3 => Ok(Action::Editing),
            4 => Ok(Action::Modding),
            5 => Ok(Action::Multiplayer),
            6 => Ok(Action::Watching),
            7 => Ok(Action::Ranking),
            8 => Ok(Action::Testing),
            9 => Ok(Action::Submitting),
            10 => Ok(Action::Paused),
            11 => Ok(Action::Lobby),
            12 => Ok(Action::Multiplaying),
            13 => Ok(Action::Direct),
            _ => Err(Self::Error::new(ErrorKind::InvalidData, "invalid action")),
        }
    }
}

#[derive(Debug, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct UserAction<'a> {
    pub action: Action,
    pub info_text: &'a str,
    pub beatmap_md5: &'a str,
    pub mods: Mods,
    pub mode: Mode,
    pub beatmap_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::{BinarySerialize, BinaryDeserialize};

    fn test_user_action_roundtrip(action: &UserAction) {
        let serialized = action.serialize();
        let deserialized = UserAction::deserialize(&serialized).unwrap();
        assert_eq!(action.action, deserialized.action);
        assert_eq!(action.info_text, deserialized.info_text);
        assert_eq!(action.beatmap_md5, deserialized.beatmap_md5);
        assert_eq!(action.mods, deserialized.mods);
        assert_eq!(action.mode, deserialized.mode);
        assert_eq!(action.beatmap_id, deserialized.beatmap_id);
    }

    #[test]
    fn test_action_wire_values() {
        use crate::serde::BinarySerialize;
        let expected: &[(Action, u8)] = &[
            (Action::Idle, 0),
            (Action::AFK, 1),
            (Action::Playing, 2),
            (Action::Editing, 3),
            (Action::Modding, 4),
            (Action::Multiplayer, 5),
            (Action::Watching, 6),
            (Action::Ranking, 7),
            (Action::Testing, 8),
            (Action::Submitting, 9),
            (Action::Paused, 10),
            (Action::Lobby, 11),
            (Action::Multiplaying, 12),
            (Action::Direct, 13),
        ];
        for (action, byte) in expected {
            assert_eq!(action.serialize(), [*byte]);
        }
    }

    #[test]
    fn test_action_default() {
        let action: Action = Default::default();
        assert_eq!(action, Action::Idle);
    }

    #[test]
    fn test_action_try_from_valid() {
        for i in 0..=13u8 {
            let action = Action::try_from(i);
            assert!(action.is_ok());
        }
    }

    #[test]
    fn test_action_try_from_invalid() {
        use std::io::ErrorKind;
        let result = Action::try_from(14u8);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn test_action_try_from_max() {
        let result = Action::try_from(u8::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_action_idle() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let action = UserAction {
            action: Action::Idle,
            info_text: "afk",
            beatmap_md5: "",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 0,
        };
        let serialized = action.serialize();
        let deserialized = UserAction::deserialize(&serialized).unwrap();
        assert_eq!(action.action, deserialized.action);
        assert_eq!(action.info_text, deserialized.info_text);
        assert_eq!(action.beatmap_md5, deserialized.beatmap_md5);
        assert_eq!(action.mods, deserialized.mods);
        assert_eq!(action.mode, deserialized.mode);
        assert_eq!(action.beatmap_id, deserialized.beatmap_id);
    }

    #[test]
    fn test_user_action_playing() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "playing",
            beatmap_md5: "abc123def456",
            mods: Mods::Hidden | Mods::Doubletime,
            mode: Mode::Standard,
            beatmap_id: 123456,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_editing() {
        let action = UserAction {
            action: Action::Editing,
            info_text: "[approved] Song Name [Difficulty]",
            beatmap_md5: "abcdef123456",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 654321,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_modding() {
        let action = UserAction {
            action: Action::Modding,
            info_text: "[pending] Beatmap [Diff]",
            beatmap_md5: "xyz789",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 111111,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_multiplayer() {
        let action = UserAction {
            action: Action::Multiplayer,
            info_text: "multiplayer",
            beatmap_md5: "mp123",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 0,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_watching() {
        let action = UserAction {
            action: Action::Watching,
            info_text: "watching",
            beatmap_md5: "watched123",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 222222,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_taiko_mode() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "playing taiko",
            beatmap_md5: "taiko_map",
            mods: Mods::None,
            mode: Mode::Taiko,
            beatmap_id: 333333,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_catch_mode() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "playing catch",
            beatmap_md5: "catch_map",
            mods: Mods::None,
            mode: Mode::Catch,
            beatmap_id: 444444,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_mania_mode() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "playing mania",
            beatmap_md5: "mania_map",
            mods: Mods::None,
            mode: Mode::Mania,
            beatmap_id: 555555,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_no_fail_mod() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "easy playing",
            beatmap_md5: "easy_map",
            mods: Mods::NoFail | Mods::Easy,
            mode: Mode::Standard,
            beatmap_id: 666666,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_multiple_mods() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "hard playing",
            beatmap_md5: "hard_map",
            mods: Mods::HardRock | Mods::SuddenDeath | Mods::Perfect,
            mode: Mode::Standard,
            beatmap_id: 777777,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_empty_text() {
        let action = UserAction {
            action: Action::Idle,
            info_text: "",
            beatmap_md5: "",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 0,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_unicode_text() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "プレイ中 [日本語マップ]",
            beatmap_md5: "unicode_map",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: 888888,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_negative_beatmap_id() {
        let action = UserAction {
            action: Action::Idle,
            info_text: "testing",
            beatmap_md5: "",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: -1,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_user_action_large_beatmap_id() {
        let action = UserAction {
            action: Action::Playing,
            info_text: "playing",
            beatmap_md5: "some_md5",
            mods: Mods::None,
            mode: Mode::Standard,
            beatmap_id: i32::MAX,
        };
        test_user_action_roundtrip(&action);
    }

    #[test]
    fn test_action_deserialize_invalid() {
        use crate::serde::BinaryDeserialize;
        let data = [14u8]; // invalid: max valid is 13 (Direct)
        assert!(Action::deserialize(&data).is_err());
    }

    #[test]
    fn test_action_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(Action::deserialize(&data).is_err());
    }

    #[test]
    fn test_action_serde_roundtrip() {
        let actions = [
            Action::Idle,
            Action::AFK,
            Action::Playing,
            Action::Editing,
            Action::Modding,
            Action::Multiplayer,
            Action::Watching,
            Action::Ranking,
            Action::Testing,
            Action::Submitting,
            Action::Paused,
            Action::Lobby,
            Action::Multiplaying,
            Action::Direct,
        ];
        for action in actions {
            let bytes = action.serialize();
            let decoded = Action::deserialize(&bytes).unwrap();
            assert_eq!(action, decoded);
        }
    }
}
