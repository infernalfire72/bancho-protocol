use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::Pong)]
pub struct Pong;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_pong_byte_size() {
        let pong = Pong;
        assert_eq!(pong.byte_size(), 0);
    }

    #[test]
    fn test_pong_serialize() {
        let pong = Pong;
        let bytes = pong.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_pong_message_type() {
        assert_eq!(Pong::MESSAGE_TYPE, MessageType::Pong);
    }
}
