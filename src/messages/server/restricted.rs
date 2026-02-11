use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::Restricted)]
pub struct Restricted;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_restricted_byte_size() {
        let msg = Restricted;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_restricted_serialize() {
        let msg = Restricted;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_restricted_message_type() {
        assert_eq!(Restricted::MESSAGE_TYPE, MessageType::Restricted);
    }

    #[test]
    fn test_restricted_debug_format() {
        let msg = Restricted;
        assert_eq!(format!("{:?}", msg), "Restricted");
    }
}
