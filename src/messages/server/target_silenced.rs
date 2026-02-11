use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::TargetSilenced)]
pub struct TargetSilenced<'a> {
    placeholder: u8,
    placeholder2: u8,
    target: &'a str,
    placeholder3: i32,
}

impl<'a> TargetSilenced<'a> {
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
    fn test_target_silenced_new() {
        let msg = TargetSilenced::new("someuser");
        assert_eq!(msg.target, "someuser");
        assert_eq!(msg.placeholder, 0);
        assert_eq!(msg.placeholder2, 0);
        assert_eq!(msg.placeholder3, 0);
    }

    #[test]
    fn test_target_silenced_byte_size() {
        let msg = TargetSilenced::new("test");
        // u8(1) + u8(1) + str("test": 1+1+4) + i32(4) = 12
        assert_eq!(msg.byte_size(), 1 + 1 + 6 + 4);
    }

    #[test]
    fn test_target_silenced_byte_size_empty_target() {
        let msg = TargetSilenced::new("");
        // u8(1) + u8(1) + str("": 1) + i32(4) = 7
        assert_eq!(msg.byte_size(), 1 + 1 + 1 + 4);
    }

    #[test]
    fn test_target_silenced_serialize() {
        let msg = TargetSilenced::new("ab");
        let bytes = msg.serialize();
        // u8(0) + u8(0) + str("ab": 0x0b, 0x02, 'a', 'b') + i32(0)
        let expected: Vec<u8> = vec![
            0x00,                   // placeholder
            0x00,                   // placeholder2
            0x0b, 0x02, b'a', b'b', // target "ab"
            0x00, 0x00, 0x00, 0x00, // placeholder3
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_target_silenced_message_type() {
        assert_eq!(TargetSilenced::MESSAGE_TYPE, MessageType::TargetSilenced);
    }
}
