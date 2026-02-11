use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MainMenuIcon)]
pub struct MainMenuIcon {
    format: String,
}

impl MainMenuIcon {
    pub fn new<'a>(icon_url: &'a str, onclick_url: &'a str) -> Self {
        Self {
            format: format!("{}|{}", icon_url, onclick_url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;
    use crate::serde::BinarySerialize;

    #[test]
    fn test_main_menu_icon_new_format() {
        let msg = MainMenuIcon::new("http://icon.png", "http://click.me");
        assert_eq!(msg.format, "http://icon.png|http://click.me");
    }

    #[test]
    fn test_main_menu_icon_new_empty_urls() {
        let msg = MainMenuIcon::new("", "");
        assert_eq!(msg.format, "|");
    }

    #[test]
    fn test_main_menu_icon_byte_size() {
        let msg = MainMenuIcon::new("a", "b");
        // String "a|b" (3 chars): 1 (marker) + 1 (uleb128 len) + 3 = 5
        assert_eq!(msg.byte_size(), 5);
    }

    #[test]
    fn test_main_menu_icon_serialize() {
        let msg = MainMenuIcon::new("a", "b");
        let bytes = msg.serialize();
        // String "a|b": 0x0b, uleb128(3), "a|b"
        assert_eq!(bytes[0], 0x0b);
        assert_eq!(bytes[1], 3);
        assert_eq!(&bytes[2..], b"a|b");
    }

    #[test]
    fn test_main_menu_icon_message_type() {
        assert_eq!(MainMenuIcon::MESSAGE_TYPE, MessageType::MainMenuIcon);
    }
}
