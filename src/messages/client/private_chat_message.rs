use crate::serde::macros::BinaryDeserialize;
use crate::structures::IrcMessage;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct PrivateChatMessage<'a> {
    pub message: IrcMessage<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_private_chat_message_empty() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "",
            text: "",
            recipient: "",
            sender_id: 0,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = PrivateChatMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "");
        assert_eq!(msg.message.text, "");
    }

    #[test]
    fn test_private_chat_message_with_content() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "user1",
            text: "hello",
            recipient: "user2",
            sender_id: 100,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = PrivateChatMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "user1");
        assert_eq!(msg.message.recipient, "user2");
        assert_eq!(msg.message.text, "hello");
        assert_eq!(msg.message.sender_id, 100);
    }
}
