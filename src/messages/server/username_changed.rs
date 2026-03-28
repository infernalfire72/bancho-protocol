use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use bancho_protocol_macros::BinaryDeserialize;

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UsernameChanged)]
pub struct UsernameChanged {
    username_change: String,
}

impl UsernameChanged {
    pub fn new(old_username: &str, new_username: &str) -> Self {
        UsernameChanged {
            username_change: format!("{}>>>>{}", old_username, new_username),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;
    use crate::serde::{BinaryDeserialize, BinarySerialize};

    #[test]
    fn test_username_changed_new_format() {
        let msg = UsernameChanged::new("oldname", "newname");
        assert_eq!(msg.username_change, "oldname>>>>newname");
    }

    #[test]
    fn test_username_changed_new_empty_names() {
        let msg = UsernameChanged::new("", "");
        assert_eq!(msg.username_change, ">>>>");
    }

    #[test]
    fn test_username_changed_byte_size() {
        let msg = UsernameChanged::new("ab", "cd");
        // String "ab>>>>cd" (8 chars): 1 (marker) + 1 (uleb128 len) + 8 = 10
        assert_eq!(msg.byte_size(), 10);
    }

    #[test]
    fn test_username_changed_serialize() {
        let msg = UsernameChanged::new("a", "b");
        let bytes = msg.serialize();
        // String "a>>>>b" (6 chars): 0x0b, uleb128(6), "a>>>>b"
        assert_eq!(bytes[0], 0x0b);
        assert_eq!(bytes[1], 6);
        assert_eq!(&bytes[2..], b"a>>>>b");
    }

    #[test]
    fn test_username_changed_serialize_deserialize_roundtrip() {
        let original = UsernameChanged::new("olduser", "newuser");
        let bytes = original.serialize();
        let deserialized = UsernameChanged::deserialize(&bytes).unwrap();
        assert_eq!(deserialized.username_change, "olduser>>>>newuser");
    }

    #[test]
    fn test_username_changed_message_type() {
        assert_eq!(
            UsernameChanged::MESSAGE_TYPE,
            MessageType::UsernameChanged
        );
    }
}
