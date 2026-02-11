use crate::serde::macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[derive(Debug, Clone, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct IrcMessage<'a> {
    pub sender: &'a str,
    pub text: &'a str,
    pub recipient: &'a str,
    pub sender_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::{BinarySerialize, BinaryDeserialize};

    #[test]
    fn test_irc_message_basic_roundtrip() {
        let msg = IrcMessage {
            sender: "player",
            text: "hello world",
            recipient: "#osu",
            sender_id: 1000,
        };
        let serialized = msg.serialize();
        let deserialized = IrcMessage::deserialize(&serialized).unwrap();
        assert_eq!(msg.sender, deserialized.sender);
        assert_eq!(msg.text, deserialized.text);
        assert_eq!(msg.recipient, deserialized.recipient);
        assert_eq!(msg.sender_id, deserialized.sender_id);
    }

    #[test]
    fn test_irc_message_unicode_sender() {
        let msg = IrcMessage {
            sender: "プレイヤー",
            text: "hello",
            recipient: "#osu",
            sender_id: 2000,
        };
        let serialized = msg.serialize();
        let deserialized = IrcMessage::deserialize(&serialized).unwrap();
        assert_eq!(msg.sender, deserialized.sender);
    }

    #[test]
    fn test_irc_message_unicode_text() {
        let msg = IrcMessage {
            sender: "player",
            text: "こんにちは世界",
            recipient: "#osu",
            sender_id: 3000,
        };
        let serialized = msg.serialize();
        let deserialized = IrcMessage::deserialize(&serialized).unwrap();
        assert_eq!(msg.text, deserialized.text);
    }

    #[test]
    fn test_irc_message_special_characters() {
        let msg = IrcMessage {
            sender: "user@host",
            text: "Hello! [test] {brackets}",
            recipient: "#special",
            sender_id: 7777,
        };
        let serialized = msg.serialize();
        let deserialized = IrcMessage::deserialize(&serialized).unwrap();
        assert_eq!(msg.sender, deserialized.sender);
        assert_eq!(msg.text, deserialized.text);
    }
}
