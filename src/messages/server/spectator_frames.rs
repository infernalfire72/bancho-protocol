use crate::messages::{Message, MessageType};
use crate::structures::ReplayFrameBundle;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::SpectatorFrames)]
pub struct SpectatorFrames<'a> {
    pub frames: &'a ReplayFrameBundle,
}