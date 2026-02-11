use super::MessageType;
use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};

pub const HEADER_SIZE: usize = 7;

#[derive(Debug)]
pub struct MessageHeader {
    pub message_type: MessageType,
    _compress: bool,
    pub args_len: u32,
}

impl<'a> BinaryDeserialize<'a> for MessageHeader {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let message_type = MessageType::read_from(reader)?;
        let _compress = bool::read_from(reader)?;
        let args_len = u32::read_from(reader)?;
        Ok(Self {
            message_type,
            _compress,
            args_len,
        })
    }
}

pub trait MessageArgs: Sized + BinarySerialize {
    const MESSAGE_TYPE: MessageType;
    fn as_message(&self) -> Message<'_, Self> {
        Message(self)
    }
}

/// A general interface for bancho packets.
/// Never construct this manually.
#[derive(Debug)]
pub struct Message<'a, Args: MessageArgs>(pub &'a Args);

impl<Args: MessageArgs> Message<'_, Args> {
    pub fn serialize(args: Args) -> Vec<u8> {
        let msg = args.as_message();
        msg.serialize()
    }
}

impl<Args: MessageArgs> ByteSized for Message<'_, Args> {
    fn byte_size(&self) -> usize {
        HEADER_SIZE + self.0.byte_size()
    }
}

impl<Args: MessageArgs> BinarySerialize for Message<'_, Args> {
    fn write_to(&self, mut writer: &mut BinaryWriter) {
        let msg_id = Args::MESSAGE_TYPE as u16;
        let args_len = self.0.byte_size() as u32;
        // header
        msg_id.write_to(&mut writer);
        writer.write_byte(0);
        args_len.write_to(&mut writer);

        self.0.write_to(&mut writer);
    }
}

impl<Args: MessageArgs> ByteSized for (Args,) {
    fn byte_size(&self) -> usize {
        self.0.as_message().byte_size()
    }
}

impl<Args: MessageArgs> BinarySerialize for (Args,) {
    fn write_to(&self, writer: &mut BinaryWriter) {
        self.0.as_message().write_to(writer);
    }
}

impl<Args: MessageArgs, B: ByteSized> ByteSized for (Args, B) {
    fn byte_size(&self) -> usize {
        self.0.as_message().byte_size() + self.1.byte_size()
    }
}

impl<Args: MessageArgs, B: BinarySerialize> BinarySerialize for (Args, B) {
    fn write_to(&self, writer: &mut BinaryWriter) {
        self.0.as_message().write_to(writer);
        self.1.write_to(writer);
    }
}

#[macro_export]
macro_rules! expand_into_tuple {
    () => { () };
    ($e:expr) => { ($e,) };
    ($e:expr, $($rest:expr),+) => {
        ($e, expand_into_tuple!($($rest),+))
    };
}

#[macro_export]
macro_rules! concat_messages {
    ($($e:expr),+ $(,)?) => {
        {
            use $crate::expand_into_tuple;
            use $crate::serde::BinarySerialize;
            expand_into_tuple!($($e),+).serialize()
        }
    };
    [$e:expr; $n:expr] => {
        {
            let msg = $e.as_message();
            let data_len = msg.byte_size() * $n;
            let mut writer = BinaryWriter::with_length(data_len);
            for _ in 0..$n {
                msg.write_to(&mut writer);
            }
            writer.data()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_header_size() {
        assert_eq!(HEADER_SIZE, 7);
    }

    #[test]
    fn test_message_header_layout() {
        // message_type (u16) + compress (bool/1) + args_len (u32)
        // = 2 + 1 + 4 = 7 bytes
        assert_eq!(HEADER_SIZE, 2 + 1 + 4);
    }

    #[test]
    fn test_message_header_parse_valid() {
        let bytes = vec![
            0x05, 0x00, // message_type = 5 (LoginResult)
            0x00,       // compress = false
            0x10, 0x00, 0x00, 0x00, // args_len = 16
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.message_type, MessageType::LoginResult);
        assert_eq!(header.args_len, 16);
    }

    #[test]
    fn test_message_header_parse_compress_true() {
        let bytes = vec![
            0x08, 0x00, // message_type = 8 (Pong)
            0x01,       // compress = true
            0x00, 0x00, 0x00, 0x00, // args_len = 0
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.message_type, MessageType::Pong);
    }

    #[test]
    fn test_message_header_parse_large_payload() {
        let bytes = vec![
            0x26, 0x00, // message_type = 38 (MatchChangeSlot)
            0x00,       // compress = false
            0xFF, 0xFF, 0x00, 0x00, // args_len = 65535
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.args_len, 65535);
    }

    #[test]
    fn test_message_header_parse_zero_payload() {
        let bytes = vec![
            0x08, 0x00, // message_type = 8 (Pong)
            0x00,       // compress = false
            0x00, 0x00, 0x00, 0x00, // args_len = 0
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.args_len, 0);
    }

    #[test]
    fn test_message_header_insufficient_bytes() {
        let bytes = vec![0x05, 0x00, 0x00]; // Only 3 bytes instead of 7
        let mut reader = BinaryReader::from(&bytes);
        let result = MessageHeader::read_from(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_header_empty_bytes() {
        let bytes: Vec<u8> = vec![];
        let mut reader = BinaryReader::from(&bytes);
        let result = MessageHeader::read_from(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_header_all_client_message_types() {
        // Test a sampling of client message types
        let client_types = vec![
            (0u16, MessageType::ChangeAction),
            (1u16, MessageType::PublicChatMessage),
            (4u16, MessageType::Ping),
            (16u16, MessageType::StartSpectating),
        ];

        for (id, expected_type) in client_types {
            let mut bytes = id.to_le_bytes().to_vec();
            bytes.push(0x00); // compress
            bytes.extend_from_slice(&0u32.to_le_bytes());
            let mut reader = BinaryReader::from(&bytes);
            let header = MessageHeader::read_from(&mut reader).unwrap();
            assert_eq!(header.message_type, expected_type);
        }
    }

    #[test]
    fn test_message_header_all_server_message_types() {
        // Test a sampling of server message types
        let server_types = vec![
            (5u16, MessageType::LoginResult),
            (7u16, MessageType::ChatMessage),
            (8u16, MessageType::Pong),
            (11u16, MessageType::UserStats),
        ];

        for (id, expected_type) in server_types {
            let mut bytes = id.to_le_bytes().to_vec();
            bytes.push(0x00); // compress
            bytes.extend_from_slice(&0u32.to_le_bytes());
            let mut reader = BinaryReader::from(&bytes);
            let header = MessageHeader::read_from(&mut reader).unwrap();
            assert_eq!(header.message_type, expected_type);
        }
    }

    #[test]
    fn test_message_header_little_endian_message_type() {
        let bytes = vec![
            0x05, 0x00, // message_type = 5 (LoginResult, little endian)
            0x00,       // compress = false
            0x00, 0x00, 0x00, 0x00, // args_len = 0
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.message_type as u16, 5);
    }

    #[test]
    fn test_message_header_little_endian_args_len() {
        let bytes = vec![
            0x08, 0x00, // message_type = 8
            0x00,       // compress = false
            0x34, 0x12, 0x00, 0x00, // args_len = 0x1234 in little endian
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.args_len, 0x1234);
    }

    #[test]
    fn test_message_header_debug_format() {
        let header = MessageHeader {
            message_type: MessageType::Pong,
            _compress: false,
            args_len: 42,
        };
        let debug_str = format!("{:?}", header);
        assert!(debug_str.contains("MessageHeader"));
        assert!(debug_str.contains("Pong") || debug_str.contains("8"));
    }

    #[test]
    fn test_expand_into_tuple_empty() {
        let result = expand_into_tuple!();
        assert_eq!(result, ());
    }

    #[test]
    fn test_message_header_maximum_args_len() {
        let bytes = vec![
            0x08, 0x00,                           // message_type = 8
            0x00,                                 // compress = false
            0xFF, 0xFF, 0xFF, 0xFF,               // args_len = u32::MAX
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();
        assert_eq!(header.args_len, u32::MAX);
    }

    #[test]
    fn test_message_header_match_update() {
        let bytes = vec![
            0x1A, 0x00, // message_type = 26 (MatchUpdate)
            0x00,       // compress = false
            0x00, 0x04, 0x00, 0x00, // args_len = 1024
        ];
        let mut reader = BinaryReader::from(&bytes);
        let header = MessageHeader::read_from(&mut reader).unwrap();

        assert_eq!(header.message_type, MessageType::MatchUpdate);
        assert_eq!(header.args_len, 1024);
    }

    // Tests for tuple write_to implementations (lines 73-75, 85-88)

    // A minimal MessageArgs type for testing
    use crate::messages::MessageType;
    use crate::serde::macros::{BinarySerialize as BinarySerializeDerive, ByteSized as ByteSizedDerive, Message as MessageDerive};

    #[derive(Debug, BinarySerializeDerive, ByteSizedDerive, MessageDerive)]
    #[crate_root(crate)]
    #[message(MessageType::Pong)]
    struct TestPong;

    #[derive(Debug, BinarySerializeDerive, ByteSizedDerive, MessageDerive)]
    #[crate_root(crate)]
    #[message(MessageType::LoginResult)]
    struct TestLoginResult {
        pub user_id: i32,
    }

    #[test]
    fn test_single_tuple_write_to() {
        // Tests (Args,) BinarySerialize::write_to
        let pong = TestPong;
        let tuple = (pong,);
        let bytes = tuple.serialize();

        // Should produce a valid message with header
        assert_eq!(bytes.len(), HEADER_SIZE);
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type, MessageType::Pong as u16);
        assert_eq!(bytes[2], 0); // compress = false
        let args_len = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]);
        assert_eq!(args_len, 0); // Pong has no payload
    }

    #[test]
    fn test_two_tuple_write_to() {
        // Tests (Args, B) BinarySerialize::write_to
        let login = TestLoginResult { user_id: 999 };
        let pong = TestPong;
        let tuple = (login, (pong,));
        let bytes = tuple.serialize();

        // First message: LoginResult header (7) + i32 payload (4)
        // Second message: Pong header (7) + no payload
        assert_eq!(bytes.len(), HEADER_SIZE + 4 + HEADER_SIZE);

        // Verify first message header
        let msg_type1 = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type1, MessageType::LoginResult as u16);
        let args_len1 = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]);
        assert_eq!(args_len1, 4);

        // Verify payload of first message
        let user_id = i32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]);
        assert_eq!(user_id, 999);

        // Verify second message header
        let msg_type2 = u16::from_le_bytes([bytes[11], bytes[12]]);
        assert_eq!(msg_type2, MessageType::Pong as u16);
    }

    #[test]
    fn test_concat_messages_single() {
        let bytes = concat_messages!(TestPong);
        assert_eq!(bytes.len(), HEADER_SIZE);
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type, MessageType::Pong as u16);
    }

    #[test]
    fn test_concat_messages_multiple() {
        let bytes = concat_messages!(
            TestLoginResult { user_id: 42 },
            TestPong
        );
        // LoginResult: 7 header + 4 payload = 11
        // Pong: 7 header + 0 payload = 7
        assert_eq!(bytes.len(), 18);
    }

    #[test]
    fn test_message_serialize_static() {
        let login = TestLoginResult { user_id: -1 };
        let bytes = Message::serialize(login);

        assert_eq!(bytes.len(), HEADER_SIZE + 4);
        let msg_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(msg_type, MessageType::LoginResult as u16);
        let args_len = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]);
        assert_eq!(args_len, 4);
        let user_id = i32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]);
        assert_eq!(user_id, -1);
    }
}
