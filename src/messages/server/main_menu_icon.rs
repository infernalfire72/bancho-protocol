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
