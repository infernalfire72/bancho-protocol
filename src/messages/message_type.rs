use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    // Client
    ChangeAction = 0,
    PublicChatMessage = 1,
    Logout = 2,
    UpdateStatsRequest = 3,
    Ping = 4,
    StartSpectating = 16,
    StopSpectating = 17,
    SpectateFrames = 18,
    CantSpectate = 21,
    PrivateChatMessage = 25,
    LeaveLobby = 29,
    JoinLobby = 30,
    CreateMatch = 31,
    JoinMatch = 32,
    LeaveMatch = 33,
    MatchChangeSlot = 38,
    MatchReady = 39,
    MatchLock = 40,
    MatchChangeSettings = 41,
    StartMatch = 44,
    UpdateMatchScore = 47,
    MatchPlayerComplete = 49,
    MatchChangeMods = 51,
    MatchLoadComplete = 52,
    MatchNoBeatmap = 54,
    MatchNotReady = 55,
    MatchFailed = 56,
    MatchHasBeatmap = 59,
    MatchSkipRequest = 60,
    JoinChannel = 63,
    MatchChangeHost = 70,
    AddFriend = 73,
    RemoveFriend = 74,
    MatchChangeTeam = 77,
    LeaveChannel = 78,
    ReceiveUpdates = 79,
    SetAwayMessage = 82,
    UserStatsRequest = 85,
    MatchInvite = 87,
    MatchChangePassword = 90,
    TournamentMatchInfoRequest = 93,
    RequestPresences = 97,
    RequestAllPresences = 98,
    ToggleBlockNonFriendDms = 99,
    TournamentJoinMatchChannel = 108,
    TournamentLeaveMatchChannel = 109,

    // Server
    LoginResult = 5,
    ChatMessage = 7,
    UsernameChanged = 9,
    Pong = 8,
    UserStats = 11,
    UserLogout = 12,
    SpectatorJoined = 13,
    SpectatorLeft = 14,
    SpectatorFrames = 15,
    FailedSpectating = 22,
    GetAttention = 23,
    Alert = 24,
    MatchUpdate = 26,
    MatchCreated = 27,
    MatchDisposed = 28,
    MatchJoinSuccess = 36,
    MatchJoinFailed = 37,
    FellowSpectatorJoined = 42,
    FellowSpectatorLeft = 43,
    MatchStart = 46,
    MatchScoreUpdate = 48,
    MatchTransferHost = 50,
    MatchAllPlayersLoaded = 53,
    MatchPlayerFailed = 57,
    MatchComplete = 58,
    MatchSkip = 61,
    ChannelJoinSuccess = 64,
    ChannelInfo = 65,
    ChannelKick = 66,
    ChannelAutoJoin = 67,
    Privileges = 71,
    FriendsList = 72,
    ProtocolVersion = 75,
    MainMenuIcon = 76,
    MatchPlayerSkipped = 81,
    UserPresence = 83,
    Restart = 86,
    ChannelInfoEnd = 89,
    MatchPasswordChanged = 91,
    SilenceEnd = 92,
    UserSilenced = 94,
    UserPresenceSingle = 95,
    UserPresenceBundle = 96,
    UserDmBlocked = 100,
    TargetSilenced = 101,
    Restricted = 104,
    MatchAborted = 106,
    SwitchServer = 107,
}

impl TryFrom<u16> for MessageType {
    type Error = std::io::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;

        Ok(match value {
            // Client
            0 => MessageType::ChangeAction,
            1 => MessageType::PublicChatMessage,
            2 => MessageType::Logout,
            3 => MessageType::UpdateStatsRequest,
            4 => MessageType::Ping,
            16 => MessageType::StartSpectating,
            17 => MessageType::StopSpectating,
            18 => MessageType::SpectateFrames,
            21 => MessageType::CantSpectate,
            25 => MessageType::PrivateChatMessage,
            29 => MessageType::LeaveLobby,
            30 => MessageType::JoinLobby,
            31 => MessageType::CreateMatch,
            32 => MessageType::JoinMatch,
            33 => MessageType::LeaveMatch,
            38 => MessageType::MatchChangeSlot,
            39 => MessageType::MatchReady,
            40 => MessageType::MatchLock,
            41 => MessageType::MatchChangeSettings,
            44 => MessageType::StartMatch,
            47 => MessageType::UpdateMatchScore,
            49 => MessageType::MatchPlayerComplete,
            51 => MessageType::MatchChangeMods,
            52 => MessageType::MatchLoadComplete,
            54 => MessageType::MatchNoBeatmap,
            55 => MessageType::MatchNotReady,
            56 => MessageType::MatchFailed,
            59 => MessageType::MatchHasBeatmap,
            60 => MessageType::MatchSkipRequest,
            63 => MessageType::JoinChannel,
            70 => MessageType::MatchChangeHost,
            73 => MessageType::AddFriend,
            74 => MessageType::RemoveFriend,
            77 => MessageType::MatchChangeTeam,
            78 => MessageType::LeaveChannel,
            79 => MessageType::ReceiveUpdates,
            82 => MessageType::SetAwayMessage,
            85 => MessageType::UserStatsRequest,
            87 => MessageType::MatchInvite,
            90 => MessageType::MatchChangePassword,
            93 => MessageType::TournamentMatchInfoRequest,
            97 => MessageType::RequestPresences,
            98 => MessageType::RequestAllPresences,
            99 => MessageType::ToggleBlockNonFriendDms,
            108 => MessageType::TournamentJoinMatchChannel,
            109 => MessageType::TournamentLeaveMatchChannel,

            // Server
            5 => MessageType::LoginResult,
            7 => MessageType::ChatMessage,
            8 => MessageType::Pong,
            9 => MessageType::UsernameChanged,
            11 => MessageType::UserStats,
            12 => MessageType::UserLogout,
            13 => MessageType::SpectatorJoined,
            14 => MessageType::SpectatorLeft,
            15 => MessageType::SpectatorFrames,
            22 => MessageType::FailedSpectating,
            23 => MessageType::GetAttention,
            24 => MessageType::Alert,
            26 => MessageType::MatchUpdate,
            27 => MessageType::MatchCreated,
            28 => MessageType::MatchDisposed,
            36 => MessageType::MatchJoinSuccess,
            37 => MessageType::MatchJoinFailed,
            42 => MessageType::FellowSpectatorJoined,
            43 => MessageType::FellowSpectatorLeft,
            46 => MessageType::MatchStart,
            48 => MessageType::MatchScoreUpdate,
            50 => MessageType::MatchTransferHost,
            53 => MessageType::MatchAllPlayersLoaded,
            57 => MessageType::MatchPlayerFailed,
            58 => MessageType::MatchComplete,
            61 => MessageType::MatchSkip,
            64 => MessageType::ChannelJoinSuccess,
            65 => MessageType::ChannelInfo,
            66 => MessageType::ChannelKick,
            67 => MessageType::ChannelAutoJoin,
            71 => MessageType::Privileges,
            72 => MessageType::FriendsList,
            75 => MessageType::ProtocolVersion,
            76 => MessageType::MainMenuIcon,
            81 => MessageType::MatchPlayerSkipped,
            83 => MessageType::UserPresence,
            86 => MessageType::Restart,
            89 => MessageType::ChannelInfoEnd,
            91 => MessageType::MatchPasswordChanged,
            92 => MessageType::SilenceEnd,
            94 => MessageType::UserSilenced,
            95 => MessageType::UserPresenceSingle,
            96 => MessageType::UserPresenceBundle,
            100 => MessageType::UserDmBlocked,
            101 => MessageType::TargetSilenced,
            104 => MessageType::Restricted,
            106 => MessageType::MatchAborted,
            107 => MessageType::SwitchServer,
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                "invalid packet id",
            ))?,
        })
    }
}

impl<'a> BinaryDeserialize<'a> for MessageType {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        Ok(MessageType::try_from(u16::read_from(reader)?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_client_change_action() {
        assert_eq!(MessageType::ChangeAction as u16, 0);
    }

    #[test]
    fn test_message_type_client_public_chat() {
        assert_eq!(MessageType::PublicChatMessage as u16, 1);
    }

    #[test]
    fn test_message_type_client_logout() {
        assert_eq!(MessageType::Logout as u16, 2);
    }

    #[test]
    fn test_message_type_client_update_stats() {
        assert_eq!(MessageType::UpdateStatsRequest as u16, 3);
    }

    #[test]
    fn test_message_type_client_ping() {
        assert_eq!(MessageType::Ping as u16, 4);
    }

    #[test]
    fn test_message_type_server_login_result() {
        assert_eq!(MessageType::LoginResult as u16, 5);
    }

    #[test]
    fn test_message_type_server_chat_message() {
        assert_eq!(MessageType::ChatMessage as u16, 7);
    }

    #[test]
    fn test_message_type_server_pong() {
        assert_eq!(MessageType::Pong as u16, 8);
    }

    #[test]
    fn test_message_type_server_user_stats() {
        assert_eq!(MessageType::UserStats as u16, 11);
    }

    #[test]
    fn test_message_type_all_client_match_messages() {
        let match_messages = vec![
            (31u16, MessageType::CreateMatch),
            (32u16, MessageType::JoinMatch),
            (33u16, MessageType::LeaveMatch),
            (38u16, MessageType::MatchChangeSlot),
            (39u16, MessageType::MatchReady),
            (40u16, MessageType::MatchLock),
            (41u16, MessageType::MatchChangeSettings),
            (44u16, MessageType::StartMatch),
        ];

        for (id, msg_type) in match_messages {
            assert_eq!(msg_type as u16, id);
        }
    }

    #[test]
    fn test_message_type_all_server_match_messages() {
        let match_messages = vec![
            (26u16, MessageType::MatchUpdate),
            (27u16, MessageType::MatchCreated),
            (28u16, MessageType::MatchDisposed),
            (36u16, MessageType::MatchJoinSuccess),
            (37u16, MessageType::MatchJoinFailed),
            (46u16, MessageType::MatchStart),
            (58u16, MessageType::MatchComplete),
        ];

        for (id, msg_type) in match_messages {
            assert_eq!(msg_type as u16, id);
        }
    }

    #[test]
    fn test_message_type_try_from_valid_client_message() {
        assert_eq!(MessageType::try_from(0).unwrap(), MessageType::ChangeAction);
        assert_eq!(MessageType::try_from(1).unwrap(), MessageType::PublicChatMessage);
        assert_eq!(MessageType::try_from(4).unwrap(), MessageType::Ping);
    }

    #[test]
    fn test_message_type_try_from_valid_server_message() {
        assert_eq!(MessageType::try_from(5).unwrap(), MessageType::LoginResult);
        assert_eq!(MessageType::try_from(7).unwrap(), MessageType::ChatMessage);
        assert_eq!(MessageType::try_from(8).unwrap(), MessageType::Pong);
    }

    #[test]
    fn test_message_type_try_from_invalid_zero() {
        // 0 is ChangeAction, not invalid
        assert!(MessageType::try_from(0).is_ok());
    }

    #[test]
    fn test_message_type_try_from_invalid_id() {
        let invalid_ids = vec![10, 19, 20, 35, 45, 62, 68, 80, 84, 88, 102, 103, 105];

        for id in invalid_ids {
            let result = MessageType::try_from(id);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
        }
    }

    #[test]
    fn test_message_type_try_from_boundary_values() {
        // Test some boundary values
        assert!(MessageType::try_from(0).is_ok()); // Minimum valid (ChangeAction)
        assert!(MessageType::try_from(109).is_ok()); // Maximum valid (TournamentLeaveMatchChannel)
        assert!(MessageType::try_from(110).is_err()); // Just beyond valid range
        assert!(MessageType::try_from(u16::MAX).is_err()); // Maximum u16
    }

    #[test]
    fn test_message_type_deserialize_valid() {
        let bytes = (5u16).to_le_bytes(); // LoginResult
        let mut reader = BinaryReader::from(&bytes);
        let msg_type = MessageType::read_from(&mut reader).unwrap();
        assert_eq!(msg_type, MessageType::LoginResult);
    }

    #[test]
    fn test_message_type_deserialize_invalid() {
        let bytes = (255u16).to_le_bytes();
        let mut reader = BinaryReader::from(&bytes);
        let result = MessageType::read_from(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_type_deserialize_insufficient_bytes() {
        let bytes = vec![0x05]; // Only 1 byte
        let mut reader = BinaryReader::from(&bytes);
        let result = MessageType::read_from(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_type_copy_clone() {
        let mt1 = MessageType::Pong;
        let mt2 = mt1;
        assert_eq!(mt1, mt2);
    }

    #[test]
    fn test_message_type_equality() {
        assert_eq!(MessageType::Pong, MessageType::Pong);
        assert_ne!(MessageType::Pong, MessageType::Ping);
    }

    #[test]
    fn test_message_type_all_variants_have_unique_ids() {
        let all_ids = vec![
            0, 1, 2, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27,
            28, 29, 30, 31, 32, 33, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 47, 48, 49, 50, 51,
            52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 63, 64, 65, 66, 67, 70, 71, 72, 73, 74, 75,
            76, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
            100, 101, 104, 106, 107, 108, 109,
        ];

        // Verify all can be converted successfully
        for id in all_ids {
            assert!(MessageType::try_from(id).is_ok());
        }
    }

    #[test]
    fn test_message_type_spectating_messages() {
        assert_eq!(MessageType::StartSpectating as u16, 16);
        assert_eq!(MessageType::StopSpectating as u16, 17);
        assert_eq!(MessageType::SpectateFrames as u16, 18);
        assert_eq!(MessageType::SpectatorJoined as u16, 13);
        assert_eq!(MessageType::SpectatorLeft as u16, 14);
        assert_eq!(MessageType::SpectatorFrames as u16, 15);
    }

    #[test]
    fn test_message_type_chat_messages() {
        assert_eq!(MessageType::PublicChatMessage as u16, 1);
        assert_eq!(MessageType::PrivateChatMessage as u16, 25);
        assert_eq!(MessageType::ChatMessage as u16, 7);
    }

    #[test]
    fn test_message_type_channel_messages() {
        assert_eq!(MessageType::JoinChannel as u16, 63);
        assert_eq!(MessageType::LeaveChannel as u16, 78);
        assert_eq!(MessageType::ChannelJoinSuccess as u16, 64);
        assert_eq!(MessageType::ChannelInfo as u16, 65);
    }

    #[test]
    fn test_message_type_roundtrip_all_valid() {
        for id in 0u16..=109u16 {
            if let Ok(msg_type) = MessageType::try_from(id) {
                assert_eq!(msg_type as u16, id);
            }
        }
    }

    #[test]
    fn test_message_type_deserialize_little_endian() {
        let msg_type_id = 5u16; // LoginResult
        let bytes = msg_type_id.to_le_bytes();
        let mut reader = BinaryReader::from(&bytes);

        let result = MessageType::read_from(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MessageType::LoginResult);
    }

    #[test]
    fn test_message_type_large_id() {
        assert!(MessageType::try_from(1000).is_err());
        assert!(MessageType::try_from(10000).is_err());
    }

    #[test]
    fn test_message_type_friend_operations() {
        assert_eq!(MessageType::AddFriend as u16, 73);
        assert_eq!(MessageType::RemoveFriend as u16, 74);
        assert_eq!(MessageType::FriendsList as u16, 72);
    }

    #[test]
    fn test_message_type_lobby_messages() {
        assert_eq!(MessageType::JoinLobby as u16, 30);
        assert_eq!(MessageType::LeaveLobby as u16, 29);
    }

    #[test]
    fn test_message_type_presence_messages() {
        assert_eq!(MessageType::UserPresence as u16, 83);
        assert_eq!(MessageType::UserPresenceSingle as u16, 95);
        assert_eq!(MessageType::UserPresenceBundle as u16, 96);
    }

    #[test]
    fn test_message_type_username_changed() {
        assert_eq!(MessageType::UsernameChanged as u16, 9);
        assert_eq!(MessageType::try_from(9u16).unwrap(), MessageType::UsernameChanged);
    }

    // Additional BinaryDeserialize coverage via read_from
    #[test]
    fn test_message_type_deserialize_match_aborted() {
        let bytes = 106u16.to_le_bytes();
        let mut reader = BinaryReader::from(&bytes);
        let msg_type = MessageType::read_from(&mut reader).unwrap();
        assert_eq!(msg_type, MessageType::MatchAborted);
    }

    #[test]
    fn test_message_type_deserialize_switch_server() {
        let bytes = 107u16.to_le_bytes();
        let mut reader = BinaryReader::from(&bytes);
        let msg_type = MessageType::read_from(&mut reader).unwrap();
        assert_eq!(msg_type, MessageType::SwitchServer);
    }

    // Test empty reader for BinaryDeserialize
    #[test]
    fn test_message_type_deserialize_empty() {
        let data: [u8; 0] = [];
        let mut reader = BinaryReader::from(&data);
        assert!(MessageType::read_from(&mut reader).is_err());
    }
}
