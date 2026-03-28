pub mod client;
pub mod message;
pub mod message_type;
pub mod server;

pub use message::{Message, MessageArgs, MessageHeader};
pub use message_type::MessageType;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concat_messages;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;

    // Test that server Pong implements MessageArgs correctly
    #[test]
    fn test_message_args_pong_has_correct_type() {
        // Pong is a simple message with no payload
        // This test verifies it compiles and works
        assert_eq!(server::Pong.as_message().byte_size(), 7); // Header size only
    }

    // Test that Alert implements MessageArgs correctly
    #[test]
    fn test_message_args_alert_serialization() {
        let alert = server::Alert { message: "Test" };
        let msg = alert.as_message();

        // Alert should have a header (7 bytes) + string payload
        // String format is: 1 byte length + string data (for short strings in osu protocol)
        let size = msg.byte_size();
        assert!(size >= 7); // At least header size
    }

    // Test that MessageArgs::as_message creates correct wrapper
    #[test]
    fn test_message_args_as_message_wrapper() {
        let pong = server::Pong;
        let _msg = pong.as_message();

        // The wrapper should preserve the MESSAGE_TYPE
        assert_eq!(server::Pong::MESSAGE_TYPE, MessageType::Pong);
    }

    // Test Message serialization for a simple message
    #[test]
    fn test_message_serialize_pong() {
        let pong = server::Pong;
        let bytes = Message::serialize(pong);

        // Minimum: header (7 bytes) + payload
        assert!(bytes.len() >= 7);

        // First 2 bytes should be message type (little endian)
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type, MessageType::Pong as u16);

        // Byte 3 should be compress flag (0 = false)
        assert_eq!(bytes[2], 0);
    }

    // Test MessageArgs trait bounds are satisfied
    #[test]
    fn test_message_args_satisfies_bounds() {
        fn assert_message_args<T: MessageArgs>() {}

        // These should compile if MessageArgs bounds are satisfied
        assert_message_args::<server::Pong>();
        assert_message_args::<server::Alert>();
    }

    // Test that MESSAGE_TYPE constants are correctly set
    #[test]
    fn test_message_type_constants_server() {
        assert_eq!(server::Pong::MESSAGE_TYPE, MessageType::Pong);
        assert_eq!(server::Alert::MESSAGE_TYPE, MessageType::Alert);
    }

    // Test message header validation
    #[test]
    fn test_message_header_created_correctly() {
        let pong = server::Pong;
        let msg = pong.as_message();

        let size = msg.byte_size();
        assert_eq!(size, 7, "Pong should be exactly 7 bytes (header only)");
    }

    // Test that serialize creates valid headers
    #[test]
    fn test_message_serialize_header_format() {
        let alert = server::Alert { message: "Hi" };
        let bytes = Message::serialize(alert);

        assert!(bytes.len() >= 7, "Message must have at least header");

        // Parse header
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        let compress = bytes[2] != 0;
        let args_len = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]);

        assert_eq!(msg_type, MessageType::Alert as u16);
        assert!(!compress);
        assert_eq!(args_len as usize, bytes.len() - 7, "args_len should match payload");
    }

    // Test single-arg tuple serialization
    #[test]
    fn test_single_message_tuple() {
        let pong = server::Pong;
        let tuple = (pong,);

        let size = tuple.byte_size();
        assert_eq!(size, 7);
    }

    // Test that MESSAGE_TYPE is a const associated constant
    #[test]
    fn test_message_type_is_const() {
        // This just verifies it can be used in const contexts
        const PONG_TYPE: MessageType = server::Pong::MESSAGE_TYPE;
        assert_eq!(PONG_TYPE, MessageType::Pong);
    }

    // Test tuple serialization with two messages
    #[test]
    fn test_two_message_tuple() {
        let pong = server::Pong;
        let alert = server::Alert { message: "X" };

        let pong_size = pong.as_message().byte_size();
        let alert_size = alert.byte_size();

        let tuple = (pong, alert);
        let size = tuple.byte_size();
        assert_eq!(size, pong_size + alert_size);
    }

    // Test single tuple write_to produces valid bytes
    #[test]
    fn test_single_tuple_write_to_bytes() {
        use crate::serde::BinarySerialize;
        let pong = server::Pong;
        let tuple = (pong,);
        let bytes = tuple.serialize();

        assert_eq!(bytes.len(), 7); // Header only
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type, MessageType::Pong as u16);
    }

    // Test two-tuple write_to produces concatenated messages
    #[test]
    fn test_two_tuple_write_to_bytes() {
        use crate::serde::BinarySerialize;
        let pong = server::Pong;
        let alert = server::Alert { message: "Y" };

        let tuple = (pong, (alert,));
        let bytes = tuple.serialize();

        // First message: Pong header (7 bytes)
        let msg_type1 = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type1, MessageType::Pong as u16);

        // Second message starts at offset 7
        let msg_type2 = u16::from_le_bytes([bytes[7], bytes[8]]);
        assert_eq!(msg_type2, MessageType::Alert as u16);
    }

    // Test concat_messages! macro with single message
    #[test]
    fn test_concat_messages_single() {
        let bytes = concat_messages!(server::Pong);
        assert_eq!(bytes.len(), 7);
    }

    // Test concat_messages! macro with multiple messages
    #[test]
    fn test_concat_messages_two() {
        let bytes = concat_messages!(
            server::Pong,
            server::LoginResult { user_id: 1 }
        );
        // Pong: 7, LoginResult: 7 + 4 = 11
        assert_eq!(bytes.len(), 18);
    }
}
