use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub enum SlotStatus {
    None,
    Empty,
    Locked,
    NotReady,
    Ready,
    MissingBeatmap,
    Playing,
    Quit,
}