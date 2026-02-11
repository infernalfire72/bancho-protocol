use crate::serde::byte_sized::ByteSized;
use crate::serde::osu_types::PrefixedVec;
use crate::serde::{BinaryDeserialize, BinaryReader, BinarySerialize, BinaryWriter};
use crate::structures::ScoreFrame;
use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use bitflags::bitflags;
use std::io::{Error, ErrorKind};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum ReplayAction {
    Standard = 0,
    NewSong = 1,
    Skip = 2,
    Completion = 3,
    Fail = 4,
    Pause = 5,
    Unpause = 6,
    SongSelect = 7,
    WatchingOther = 8,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ButtonState: u8 {
        const M1 = 1;
        const M2 = 2;
        const K1 = 4;
        const K2 = 8;
        const Smoke = 16;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub struct ReplayFrame {
    pub button_state: ButtonState,
    pub taiko_byte: u8,
    pub x: f32,
    pub y: f32,
    pub time: i32,
}

#[derive(Debug, PartialEq, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub struct ReplayFrameBundle {
    pub extra: i32,
    pub frames: PrefixedVec<i16, ReplayFrame>,
    pub action: ReplayAction,
    pub score_frame: ScoreFrame,
    pub sequence: u16,
}

impl TryFrom<u8> for ReplayAction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 8 {
            Err(Error::new(ErrorKind::InvalidData, "invalid replay action"))
        } else {
            // SAFETY: `ReplayAction` has 9 variants
            Ok(unsafe { std::mem::transmute(value) })
        }
    }
}

impl<'a> BinaryDeserialize<'a> for ButtonState {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let val = u8::read_from(reader)?;
        Ok(ButtonState::from_bits_retain(val))
    }
}

impl ByteSized for ButtonState {
    fn byte_size(&self) -> usize {
        size_of::<u8>()
    }
}

impl BinarySerialize for ButtonState {
    fn write_to(&self, writer: &mut BinaryWriter) {
        writer.write_byte(self.bits());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_action_standard() {
        let action = ReplayAction::Standard;
        assert_eq!(action as u8, 0);
    }

    #[test]
    fn test_button_state_empty() {
        let state = ButtonState::empty();
        assert_eq!(state.bits(), 0);
    }

    #[test]
    fn test_button_state_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let state = ButtonState::M1 | ButtonState::K2 | ButtonState::Smoke;
        let serialized = state.serialize();
        let deserialized = ButtonState::deserialize(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_button_state_read_from_directly() {
        let data = [0x05u8]; // M1 | K1
        let mut reader = BinaryReader::from(data.as_slice());
        let state = ButtonState::read_from(&mut reader).unwrap();
        assert_eq!(state, ButtonState::M1 | ButtonState::K1);
    }

    #[test]
    fn test_replay_action_deserialize_invalid() {
        use crate::serde::BinaryDeserialize;
        let data = [9u8]; // invalid: max valid is 8
        assert!(ReplayAction::deserialize(&data).is_err());
    }

    #[test]
    fn test_button_state_read_from_eof() {
        let data: [u8; 0] = [];
        let mut reader = BinaryReader::from(data.as_slice());
        assert!(ButtonState::read_from(&mut reader).is_err());
    }

    #[test]
    fn test_replay_action_deserialize_eof() {
        use crate::serde::BinaryDeserialize;
        let data: [u8; 0] = [];
        assert!(ReplayAction::deserialize(&data).is_err());
    }

    #[test]
    fn test_replay_frame_basic() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frame = ReplayFrame {
            button_state: ButtonState::M1,
            taiko_byte: 0,
            x: 0.0,
            y: 0.0,
            time: 0,
        };
        let serialized = frame.serialize();
        let deserialized = ReplayFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_replay_frame_with_buttons() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frame = ReplayFrame {
            button_state: ButtonState::M1 | ButtonState::K1,
            taiko_byte: 5,
            x: 256.0,
            y: 192.0,
            time: 1000,
        };
        let serialized = frame.serialize();
        let deserialized = ReplayFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_replay_frame_max_coords() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frame = ReplayFrame {
            button_state: ButtonState::all(),
            taiko_byte: 255,
            x: f32::MAX,
            y: f32::MAX,
            time: i32::MAX,
        };
        let serialized = frame.serialize();
        let deserialized = ReplayFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_replay_frame_negative_coords() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frame = ReplayFrame {
            button_state: ButtonState::empty(),
            taiko_byte: 0,
            x: -100.5,
            y: -200.75,
            time: -5000,
        };
        let serialized = frame.serialize();
        let deserialized = ReplayFrame::deserialize(&serialized).unwrap();
        assert_eq!(frame, deserialized);
    }

    #[test]
    fn test_replay_frame_byte_size() {
        use crate::serde::byte_sized::ByteSized;
        let frame = ReplayFrame {
            button_state: ButtonState::M1,
            taiko_byte: 0,
            x: 0.0,
            y: 0.0,
            time: 0,
        };
        let expected_size = 1 + 1 + 4 + 4 + 4;
        assert_eq!(frame.byte_size(), expected_size);
    }

    #[test]
    fn test_replay_frame_bundle_empty() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let bundle = ReplayFrameBundle {
            extra: 0,
            frames: PrefixedVec::new(),
            action: ReplayAction::Standard,
            score_frame: ScoreFrame::default(),
            sequence: 0,
        };
        let serialized = bundle.serialize();
        let deserialized = ReplayFrameBundle::deserialize(&serialized).unwrap();
        assert_eq!(bundle, deserialized);
    }

    #[test]
    fn test_replay_frame_bundle_single_frame() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frame = ReplayFrame {
            button_state: ButtonState::M1,
            taiko_byte: 0,
            x: 100.0,
            y: 100.0,
            time: 500,
        };
        let bundle = ReplayFrameBundle {
            extra: 5,
            frames: PrefixedVec::from(vec![frame]),
            action: ReplayAction::Standard,
            score_frame: ScoreFrame::default(),
            sequence: 1,
        };
        let serialized = bundle.serialize();
        let deserialized = ReplayFrameBundle::deserialize(&serialized).unwrap();
        assert_eq!(bundle, deserialized);
    }

    #[test]
    fn test_replay_frame_bundle_multiple_frames() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let frames = vec![
            ReplayFrame {
                button_state: ButtonState::M1,
                taiko_byte: 0,
                x: 100.0,
                y: 100.0,
                time: 500,
            },
            ReplayFrame {
                button_state: ButtonState::K1,
                taiko_byte: 5,
                x: 200.0,
                y: 150.0,
                time: 1000,
            },
        ];
        let bundle = ReplayFrameBundle {
            extra: 10,
            frames: PrefixedVec::from(frames),
            action: ReplayAction::Completion,
            score_frame: ScoreFrame::default(),
            sequence: 42,
        };
        let serialized = bundle.serialize();
        let deserialized = ReplayFrameBundle::deserialize(&serialized).unwrap();
        assert_eq!(bundle, deserialized);
    }

    #[test]
    fn test_replay_frame_bundle_fail_action() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let bundle = ReplayFrameBundle {
            extra: 20,
            frames: PrefixedVec::new(),
            action: ReplayAction::Fail,
            score_frame: ScoreFrame::default(),
            sequence: 999,
        };
        let serialized = bundle.serialize();
        let deserialized = ReplayFrameBundle::deserialize(&serialized).unwrap();
        assert_eq!(bundle, deserialized);
    }

    // ReplayAction TryFrom tests
    #[test]
    fn test_replay_action_try_from_all_valid() {
        let expected = [
            ReplayAction::Standard,
            ReplayAction::NewSong,
            ReplayAction::Skip,
            ReplayAction::Completion,
            ReplayAction::Fail,
            ReplayAction::Pause,
            ReplayAction::Unpause,
            ReplayAction::SongSelect,
            ReplayAction::WatchingOther,
        ];
        for (i, action) in expected.iter().enumerate() {
            assert_eq!(ReplayAction::try_from(i as u8).unwrap(), *action);
        }
    }

    #[test]
    fn test_replay_action_try_from_invalid() {
        assert!(ReplayAction::try_from(9).is_err());
        assert!(ReplayAction::try_from(255).is_err());
    }

    // ReplayAction serde roundtrip
    #[test]
    fn test_replay_action_serde_roundtrip() {
        use crate::serde::{BinarySerialize, BinaryDeserialize};
        let actions = [
            ReplayAction::Standard,
            ReplayAction::NewSong,
            ReplayAction::Skip,
            ReplayAction::Completion,
            ReplayAction::Fail,
            ReplayAction::Pause,
            ReplayAction::Unpause,
            ReplayAction::SongSelect,
            ReplayAction::WatchingOther,
        ];
        for action in actions {
            let bytes = action.serialize();
            let decoded = ReplayAction::deserialize(&bytes).unwrap();
            assert_eq!(action, decoded);
        }
    }
}
