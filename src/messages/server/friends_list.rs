use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};
use crate::serde::osu_types::PrefixedVec;

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::FriendsList)]
pub struct FriendsList {
    friends: PrefixedVec<i16, i32>,
}

impl From<Vec<i32>> for FriendsList {
    fn from(value: Vec<i32>) -> Self {
        Self {
            friends: PrefixedVec::from(value),
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
    fn test_friends_list_from_empty_vec() {
        let msg = FriendsList::from(vec![]);
        assert_eq!(msg.friends, PrefixedVec::from(vec![]));
    }

    #[test]
    fn test_friends_list_from_vec() {
        let msg = FriendsList::from(vec![1, 2, 3]);
        assert_eq!(msg.friends, PrefixedVec::from(vec![1, 2, 3]));
    }

    #[test]
    fn test_friends_list_byte_size_empty() {
        let msg = FriendsList::from(vec![]);
        // i16 prefix (2 bytes) + 0 elements = 2
        assert_eq!(msg.byte_size(), 2);
    }

    #[test]
    fn test_friends_list_byte_size_with_friends() {
        let msg = FriendsList::from(vec![10, 20, 30]);
        // i16 prefix (2) + 3 * i32 (12) = 14
        assert_eq!(msg.byte_size(), 14);
    }

    #[test]
    fn test_friends_list_serialize_empty() {
        let msg = FriendsList::from(vec![]);
        let bytes = msg.serialize();
        assert_eq!(bytes, vec![0x00, 0x00]); // i16(0) LE
    }

    #[test]
    fn test_friends_list_serialize_with_friends() {
        let msg = FriendsList::from(vec![1, 2]);
        let bytes = msg.serialize();
        let mut expected = Vec::new();
        expected.extend_from_slice(&2i16.to_le_bytes()); // prefix = 2
        expected.extend_from_slice(&1i32.to_le_bytes()); // friend 1
        expected.extend_from_slice(&2i32.to_le_bytes()); // friend 2
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_friends_list_message_type() {
        assert_eq!(FriendsList::MESSAGE_TYPE, MessageType::FriendsList);
    }
}
