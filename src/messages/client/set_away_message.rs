use crate::serde::macros::BinaryDeserialize;
use crate::structures::IrcMessage;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct SetAwayMessage<'a> {
    pub message: IrcMessage<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_set_away_message_empty() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "",
            text: "",
            recipient: "",
            sender_id: 0,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = SetAwayMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "");
        assert_eq!(msg.message.text, "");
    }

    #[test]
    fn test_set_away_message_with_text() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "away",
            text: "afk now",
            recipient: "bot",
            sender_id: 100,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = SetAwayMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "away");
        assert_eq!(msg.message.recipient, "bot");
        assert_eq!(msg.message.text, "afk now");
        assert_eq!(msg.message.sender_id, 100);
    }

    #[test]
    fn test_set_away_message_debug_format() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "test",
            text: "message",
            recipient: "channel",
            sender_id: 42,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = SetAwayMessage::deserialize(&irc_bytes).unwrap();
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("SetAwayMessage"));
    }
}
