use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};
use crate::structures::{Mod, Mode, MatchTeamType, WinCondition};
use crate::structures::MatchSlot;

// TODO: proper generics and lifetimes in deserzialize
#[derive(Debug, BinaryDeserialize, BinarySerialize, ByteSized, Clone)]
#[crate_root(crate)]
pub struct Match {
    pub id: i16,
    pub in_progress: bool,
    pub powerplay: bool,
    pub mods: Mod,

    pub name: String,
    pub password: String,
    pub beatmap_name: String,
    pub beatmap_id: i32,
    pub beatmap_md5: String,
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