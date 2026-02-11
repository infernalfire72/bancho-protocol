use crate::serde::macros::BinaryDeserialize;
use crate::structures::IrcMessage;

// fix tuple structs for deserialize
#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct PublicChatMessage<'a> {
    pub message: IrcMessage<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_public_chat_message_empty() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "",
            text: "",
            recipient: "",
            sender_id: 0,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = PublicChatMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "");
        assert_eq!(msg.message.text, "");
    }

    #[test]
    fn test_public_chat_message_with_data() {
        use crate::serde::BinarySerialize;
        use crate::structures::IrcMessage;

        let irc_msg = IrcMessage {
            sender: "IRC",
            text: "hello",
            recipient: "user",
            sender_id: 42,
        };
        let irc_bytes = irc_msg.serialize();
        let msg = PublicChatMessage::deserialize(&irc_bytes).unwrap();
        assert_eq!(msg.message.sender, "IRC");
        assert_eq!(msg.message.recipient, "user");
        assert_eq!(msg.message.text, "hello");
        assert_eq!(msg.message.sender_id, 42);
    }
}
