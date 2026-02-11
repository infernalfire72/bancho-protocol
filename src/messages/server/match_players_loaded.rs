use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::MatchAllPlayersLoaded)]
pub struct MatchAllPlayersLoaded;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinarySerialize;
use crate::serde::byte_sized::ByteSized;
    use crate::messages::message::MessageArgs;

    #[test]
    fn test_match_all_players_loaded_byte_size() {
        let msg = MatchAllPlayersLoaded;
        assert_eq!(msg.byte_size(), 0);
    }

    #[test]
    fn test_match_all_players_loaded_serialize() {
        let msg = MatchAllPlayersLoaded;
        let bytes = msg.serialize();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_match_all_players_loaded_message_type() {
        assert_eq!(MatchAllPlayersLoaded::MESSAGE_TYPE, MessageType::MatchAllPlayersLoaded);
    }
}
