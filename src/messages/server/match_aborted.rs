use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchAborted)]
pub struct MatchAborted;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_match_aborted_byte_size() {
        let msg = MatchAborted;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_match_aborted_serialize() {
        let msg = MatchAborted;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_match_aborted_message_type() {
        assert_eq!(MatchAborted::MESSAGE_TYPE, MessageType::MatchAborted);
    }
}
