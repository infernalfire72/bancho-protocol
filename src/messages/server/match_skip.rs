use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchSkip)]
pub struct MatchSkip;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_match_skip_byte_size() {
        let msg = MatchSkip;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_match_skip_serialize() {
        let msg = MatchSkip;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_match_skip_message_type() {
        assert_eq!(MatchSkip::MESSAGE_TYPE, MessageType::MatchSkip);
    }
}
