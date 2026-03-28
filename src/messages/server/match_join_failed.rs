use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchJoinFailed)]
pub struct MatchJoinFailed;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_match_join_failed_byte_size() {
        let msg = MatchJoinFailed;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_match_join_failed_serialize() {
        let msg = MatchJoinFailed;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_match_join_failed_message_type() {
        assert_eq!(MatchJoinFailed::MESSAGE_TYPE, MessageType::MatchJoinFailed);
    }
}
