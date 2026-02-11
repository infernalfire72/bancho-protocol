use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchComplete)]
pub struct MatchComplete;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_match_complete_byte_size() {
        let msg = MatchComplete;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_match_complete_serialize() {
        let msg = MatchComplete;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_match_complete_message_type() {
        assert_eq!(MatchComplete::MESSAGE_TYPE, MessageType::MatchComplete);
    }

    #[test]
    fn test_match_complete_debug_format() {
        let msg = MatchComplete;
        assert_eq!(format!("{:?}", msg), "MatchComplete");
    }
}
