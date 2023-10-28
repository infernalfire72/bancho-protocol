use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum MatchTeam {
    None,
    Blue,
    Red,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum MatchTeamType {
    HeadToHead,
    TagCoop,
    Vs,
    TagVs,
}