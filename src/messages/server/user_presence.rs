use crate::messages::MessageType;
use crate::serde::osu_types::PrefixedVec;
use crate::structures::{Country, Mode, Privileges};

use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresence)]
pub struct UserPresence<'a> {
    user_id: i32,
    username: &'a str,
    timezone: u8,
    country: Country,
    privileges_gamemode: u8,
    longitude: f32,
    latitude: f32,
    global_rank: i32,
}

impl<'a> UserPresence<'a> {
    pub const fn new(
        user_id: i32,
        username: &'a str,
        timezone: i8,
        country: Country,
        gamemode: Mode,
        privileges: Privileges,
        latitude: f32,
        longitude: f32,
    ) -> Self {
        let timezone = (timezone + 24) as _;
        let privileges_gamemode = ((gamemode as u8) << 5) | ((privileges.bits() as u8) & 0x1f);
        Self {
            user_id,
            username,
            timezone,
            country,
            privileges_gamemode,
            latitude,
            longitude,
            global_rank: 0,
        }
    }
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresenceSingle)]
pub struct UserPresenceSingle {
    pub user_id: i32,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresenceBundle)]
pub struct UserPresenceBundle {
    pub user_ids: PrefixedVec<i16, i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;
    use crate::serde::BinarySerialize;
    use crate::structures::{Country, Mode, Privileges};

    #[test]
    fn test_user_presence_serialize_basic() {
        let presence = UserPresence::new(
            1000,
            "ab",
            0,
            Country::Unknown,
            Mode::Standard,
            Privileges::Player,
            0.0,
            0.0,
        );
        let bytes = presence.serialize();
        // user_id: i32(1000) LE
        assert_eq!(&bytes[0..4], &1000i32.to_le_bytes());
        // username: "ab" = 0x0b + uleb(2) + b"ab"
        assert_eq!(&bytes[4..8], &[0x0b, 2, b'a', b'b']);
        // timezone: 0 + 24 = 24
        assert_eq!(bytes[8], 24);
        // country: Unknown = 0
        assert_eq!(bytes[9], 0);
        // privileges_gamemode: Standard(0) << 5 | Player(1) & 0x1f = 1
        assert_eq!(bytes[10], 1);
        // longitude: 0.0
        assert_eq!(&bytes[11..15], &0.0f32.to_le_bytes());
        // latitude: 0.0
        assert_eq!(&bytes[15..19], &0.0f32.to_le_bytes());
        // global_rank: 0
        assert_eq!(&bytes[19..23], &0i32.to_le_bytes());
    }

    #[test]
    fn test_user_presence_timezone_negative() {
        let presence = UserPresence::new(
            1, "a", -12, Country::Unknown, Mode::Standard, Privileges::None, 0.0, 0.0,
        );
        let bytes = presence.serialize();
        // timezone byte is at offset 7 (i32 + 0x0b + uleb(1) + 'a')
        assert_eq!(bytes[7], 12); // -12 + 24
    }

    #[test]
    fn test_user_presence_timezone_positive() {
        let presence = UserPresence::new(
            1, "a", 12, Country::Unknown, Mode::Standard, Privileges::None, 0.0, 0.0,
        );
        let bytes = presence.serialize();
        assert_eq!(bytes[7], 36); // 12 + 24
    }

    #[test]
    fn test_user_presence_privileges_gamemode_encoding() {
        // Mode::Taiko (1) << 5 | Privileges::Player (1) & 0x1f = 0x21
        let presence = UserPresence::new(
            1, "a", 0, Country::Unknown, Mode::Taiko, Privileges::Player, 0.0, 0.0,
        );
        let bytes = presence.serialize();
        // privileges_gamemode byte is at offset 9 (i32 + str("a":3) + tz + country)
        assert_eq!(bytes[9], (1 << 5) | 1);
    }

    #[test]
    fn test_user_presence_privileges_gamemode_mania_supporter() {
        // Mode::Mania (3) << 5 | Privileges::Supporter (4) & 0x1f = 0x64
        let presence = UserPresence::new(
            1, "a", 0, Country::Unknown, Mode::Mania, Privileges::Supporter, 0.0, 0.0,
        );
        let bytes = presence.serialize();
        assert_eq!(bytes[9], (3 << 5) | 4);
    }

    #[test]
    fn test_user_presence_byte_size() {
        let presence = UserPresence::new(
            1000,
            "test",
            0,
            Country::Unknown,
            Mode::Standard,
            Privileges::Player,
            35.0,
            139.0,
        );
        // i32(4) + str("test": 1+1+4) + u8(1) + Country(1) + u8(1) + f32(4) + f32(4) + i32(4)
        let expected = 4 + 6 + 1 + 1 + 1 + 4 + 4 + 4;
        assert_eq!(presence.byte_size(), expected);
    }

    #[test]
    fn test_user_presence_serialize() {
        let presence = UserPresence::new(
            1,
            "ab",
            0,
            Country::Unknown,
            Mode::Standard,
            Privileges::Player,
            0.0,
            0.0,
        );
        let bytes = presence.serialize();
        // i32(1) = [0x01, 0x00, 0x00, 0x00]
        assert_eq!(&bytes[0..4], &1i32.to_le_bytes());
        // str "ab" = [0x0b, 0x02, b'a', b'b']
        assert_eq!(bytes[4], 0x0b);
        assert_eq!(bytes[5], 2); // uleb128(2)
        assert_eq!(&bytes[6..8], b"ab");
    }

    #[test]
    fn test_user_presence_message_type() {
        assert_eq!(UserPresence::MESSAGE_TYPE, MessageType::UserPresence);
    }

    #[test]
    fn test_user_presence_single_byte_size() {
        let msg = UserPresenceSingle { user_id: 42 };
        assert_eq!(msg.byte_size(), 4); // i32
    }

    #[test]
    fn test_user_presence_single_serialize() {
        let msg = UserPresenceSingle { user_id: 42 };
        let bytes = msg.serialize();
        assert_eq!(bytes, 42i32.to_le_bytes());
    }

    #[test]
    fn test_user_presence_single_message_type() {
        assert_eq!(
            UserPresenceSingle::MESSAGE_TYPE,
            MessageType::UserPresenceSingle
        );
    }

    #[test]
    fn test_user_presence_bundle_empty() {
        let msg = UserPresenceBundle {
            user_ids: PrefixedVec::new(),
        };
        // i16 prefix (2 bytes) + 0 elements
        assert_eq!(msg.byte_size(), 2);
        let bytes = msg.serialize();
        assert_eq!(bytes, vec![0x00, 0x00]); // i16(0) LE
    }

    #[test]
    fn test_user_presence_bundle_with_ids() {
        let msg = UserPresenceBundle {
            user_ids: PrefixedVec::from(vec![1, 2, 3]),
        };
        // i16 prefix (2) + 3 * i32 (12) = 14
        assert_eq!(msg.byte_size(), 14);
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 14);
        // prefix = 3 as i16 LE
        assert_eq!(&bytes[0..2], &3i16.to_le_bytes());
        // first id = 1 as i32 LE
        assert_eq!(&bytes[2..6], &1i32.to_le_bytes());
    }

    #[test]
    fn test_user_presence_bundle_message_type() {
        assert_eq!(
            UserPresenceBundle::MESSAGE_TYPE,
            MessageType::UserPresenceBundle
        );
    }
}
