use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::GetAttention)]
pub struct GetAttention;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_get_attention_byte_size() {
        let msg = GetAttention;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_get_attention_serialize() {
        let msg = GetAttention;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_get_attention_message_type() {
        assert_eq!(GetAttention::MESSAGE_TYPE, MessageType::GetAttention);
    }

    #[test]
    fn test_get_attention_debug_format() {
        let msg = GetAttention;
        assert_eq!(format!("{:?}", msg), "GetAttention");
    }
}
