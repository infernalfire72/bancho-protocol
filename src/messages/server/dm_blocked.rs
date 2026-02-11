use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresence)]
pub struct UserDmBlocked<'a> {
    placeholder: u8,
    placeholder2: u8,
    target: &'a str,
    placeholder3: i32,
}

impl<'a> UserDmBlocked<'a> {
    pub fn new(target: &'a str) -> Self {
        Self {
            target,
            placeholder: 0,
            placeholder2: 0,
            placeholder3: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;
    use crate::serde::BinarySerialize;

    #[test]
    fn test_user_dm_blocked_new() {
        let msg = UserDmBlocked::new("blockeduser");
        assert_eq!(msg.target, "blockeduser");
        assert_eq!(msg.placeholder, 0);
        assert_eq!(msg.placeholder2, 0);
        assert_eq!(msg.placeholder3, 0);
    }

    #[test]
    fn test_user_dm_blocked_byte_size() {
        let msg = UserDmBlocked::new("test");
        // u8(1) + u8(1) + str("test": 1+1+4) + i32(4) = 12
        assert_eq!(msg.byte_size(), 1 + 1 + 6 + 4);
    }

    #[test]
    fn test_user_dm_blocked_byte_size_empty_target() {
        let msg = UserDmBlocked::new("");
        // u8(1) + u8(1) + str("": 1) + i32(4) = 7
        assert_eq!(msg.byte_size(), 1 + 1 + 1 + 4);
    }

    #[test]
    fn test_user_dm_blocked_serialize() {
        let msg = UserDmBlocked::new("ab");
        let bytes = msg.serialize();
        let expected: Vec<u8> = vec![
            0x00,                   // placeholder
            0x00,                   // placeholder2
            0x0b, 0x02, b'a', b'b', // target "ab"
            0x00, 0x00, 0x00, 0x00, // placeholder3
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_user_dm_blocked_message_type() {
        assert_eq!(UserDmBlocked::MESSAGE_TYPE, MessageType::UserPresence);
    }
}
