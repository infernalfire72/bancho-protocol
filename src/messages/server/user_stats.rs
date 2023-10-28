use crate::messages::{Message, MessageType};
use crate::structures::{Action, Mod, Mode};
use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserStats)]
pub struct UserStats<'a> {
    pub user_id: i32,
    pub action: Action,
    pub info_text: &'a str,
    pub beatmap_md5: &'a str,
    pub mods: Mod,
    pub mode: Mode,
    pub beatmap_id: i32,
    pub ranked_score: i64,
    pub accuracy: f32,
    pub plays: i32,
    pub total_score: i64,
    pub global_rank: i32,
    pub performance: i16,
}