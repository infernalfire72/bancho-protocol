use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use crate::structures::{Mod, Mode, MatchTeamType, WinCondition};
use crate::structures::MatchSlot;

#[derive(Debug, BinaryDeserialize, BinarySerialize, ByteSized, Clone)]
#[crate_root(crate)]
pub struct Match<'a> {
    pub id: i16,
    pub in_progress: bool,
    pub powerplay: bool,
    pub mods: Mod,

    pub name: &'a str,
    pub password: &'a str,
    pub beatmap_name: &'a str,
    pub beatmap_id: i32,
    pub beatmap_md5: &'a str,
    pub slots: [MatchSlot; 16],

    pub host: i32,
    pub mode: Mode,
    pub win_condition: WinCondition,
    pub team_type: MatchTeamType,

    pub freemod_enabled: bool,
    #[depends(freemod_enabled)]
    pub freemods: Option<[Mod; 16]>,
    pub random_seed: i32,
}