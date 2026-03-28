use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::ChannelInfoEnd)]
pub struct ChannelInfoEnd;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_channel_info_end_byte_size() {
        let msg = ChannelInfoEnd;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_channel_info_end_serialize() {
        let msg = ChannelInfoEnd;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_channel_info_end_message_type() {
        assert_eq!(ChannelInfoEnd::MESSAGE_TYPE, MessageType::ChannelInfoEnd);
    }
}
