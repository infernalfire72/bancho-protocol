use crate::messages::{Message, MessageType};
use crate::structures::UserAction;

use crate::serde::macros::{BinarySerialize, ByteSized, Message};


#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserStats)]
pub struct UserStats<'a> {
    pub user_id: i32,
    pub action: UserAction<'a>,
    pub ranked_score: i64,
    pub accuracy: f32,
    pub plays: i32,
    pub total_score: i64,
    pub global_rank: i32,
    pub performance: i16,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserStats)]
pub struct UserStatsRef<'a>(pub &'a UserStats<'a>);