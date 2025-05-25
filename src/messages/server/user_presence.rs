use crate::messages::MessageType;
use crate::serde::osu_types::PrefixedVec;
use crate::structures::{Country, Mode, Privileges};

use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresence)]
pub struct UserPresence<'a> {
    user_id: i32,
    username: &'a str,
    timezone: u8,
    country: Country,
    privileges_gamemode: u8,
    longitude: f32,
    latitude: f32,
    global_rank: i32,
}

impl<'a> UserPresence<'a> {
    pub fn new(
        user_id: i32,
        username: &'a str,
        timezone: i8,
        country: Country,
        gamemode: Mode,
        privileges: Privileges,
        longitude: f32,
        latitude: f32,
    ) -> Self {
        let timezone = (timezone + 24) as _;
        let privileges_gamemode = ((gamemode as u8) << 5) | ((privileges.bits() as u8) & 0x1f);
        Self {
            user_id,
            username,
            timezone,
            country,
            privileges_gamemode,
            longitude,
            latitude,
            global_rank: 0,
        }
    }
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresenceSingle)]
pub struct UserPresenceSingle {
    pub user_id: i32,
}

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserPresenceBundle)]
pub struct UserPresenceBundle {
    pub user_ids: PrefixedVec<i16, i32>,
}
