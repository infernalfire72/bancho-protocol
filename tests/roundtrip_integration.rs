use bancho_protocol::serde::{BinaryDeserialize, BinarySerialize};
use bancho_protocol::serde::byte_sized::ByteSized;
use bancho_protocol::structures::*;
use bancho_protocol::messages::server::*;

// ============================================================================
// CHAT MESSAGE FLOWS
// ============================================================================

#[test]
fn test_full_chat_flow() {
    let irc = IrcMessage {
        sender: "testuser",
        text: "Hello, World!",
        recipient: "#osu",
        sender_id: 123,
    };

    let server_msg = ChatMessage(&irc);
    let serialized = server_msg.serialize();
    assert!(!serialized.is_empty());
}

// ============================================================================
// MULTIPLAYER MATCH LIFECYCLE
// ============================================================================

#[test]
fn test_match_create_flow() {
    let match_obj = Match {
        id: 1,
        in_progress: false,
        powerplay: false,
        mods: Mods::None,
        name: "Test Match",
        password: "",
        beatmap_name: "Test Beatmap",
        beatmap_id: 12345,
        beatmap_md5: "d41d8cd98f00b204e9800998ecf8427e",
        slots: [MatchSlot::default(); 16],
        host: 1000,
        mode: Mode::Standard,
        win_condition: WinCondition::Score,
        team_type: MatchTeamType::HeadToHead,
        freemod_enabled: false,
        freemods: None,
        random_seed: 0,
    };

    let serialized = match_obj.serialize();
    let deserialized = Match::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, "Test Match");
    assert_eq!(deserialized.mode, Mode::Standard);
    assert!(!deserialized.in_progress);
}

#[test]
fn test_match_with_mods() {
    let mut freemods = [Mods::None; 16];
    freemods[0] = Mods::Easy;
    freemods[1] = Mods::NoFail;

    let match_obj = Match {
        id: 42,
        in_progress: true,
        powerplay: false,
        mods: Mods::Hidden | Mods::HardRock,
        name: "HR/HD Match",
        password: "secret",
        beatmap_name: "Some Song",
        beatmap_id: 54321,
        beatmap_md5: "abc123def456",
        slots: [MatchSlot::default(); 16],
        host: 2000,
        mode: Mode::Taiko,
        win_condition: WinCondition::Accuracy,
        team_type: MatchTeamType::TagVs,
        freemod_enabled: true,
        freemods: Some(freemods),
        random_seed: 12345,
    };

    let serialized = match_obj.serialize();
    let deserialized = Match::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.id, 42);
    assert_eq!(deserialized.mods, Mods::Hidden | Mods::HardRock);
    assert!(deserialized.freemod_enabled);
    assert_eq!(deserialized.mode, Mode::Taiko);
}

// ============================================================================
// IRC MESSAGE ROUNDTRIPS
// ============================================================================

#[test]
fn test_irc_message_roundtrip() {
    let msg = IrcMessage {
        sender: "user123",
        text: "test message",
        recipient: "#channel",
        sender_id: 5000,
    };

    let serialized = msg.serialize();
    let deserialized = IrcMessage::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.sender, "user123");
    assert_eq!(deserialized.text, "test message");
    assert_eq!(deserialized.recipient, "#channel");
    assert_eq!(deserialized.sender_id, 5000);
}

#[test]
fn test_irc_message_unicode() {
    let msg = IrcMessage {
        sender: "プレイヤー",
        text: "こんにちは",
        recipient: "#日本語",
        sender_id: 6000,
    };

    let serialized = msg.serialize();
    let deserialized = IrcMessage::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.sender, "プレイヤー");
    assert_eq!(deserialized.text, "こんにちは");
}

// ============================================================================
// COMPLEX NESTED STRUCTURES
// ============================================================================

#[test]
fn test_match_slot_creation() {
    let slot = MatchSlot {
        status: SlotStatus::NotReady,
        team: MatchTeam::Red,
        user_id: 600,
    };

    assert_eq!(slot.status, SlotStatus::NotReady);
    assert_eq!(slot.team, MatchTeam::Red);
    assert_eq!(slot.user_id, 600);
}

#[test]
fn test_multiple_match_slots() {
    let mut slots = [MatchSlot::default(); 16];

    slots[0] = MatchSlot {
        status: SlotStatus::Playing,
        team: MatchTeam::Red,
        user_id: 100,
    };

    slots[1] = MatchSlot {
        status: SlotStatus::NotReady,
        team: MatchTeam::Blue,
        user_id: 200,
    };

    let serialized = slots.serialize();
    let deserialized = <[MatchSlot; 16]>::deserialize(&serialized).unwrap();

    assert_eq!(deserialized[0].status, SlotStatus::Playing);
    assert_eq!(deserialized[0].user_id, 100);
    assert_eq!(deserialized[1].status, SlotStatus::NotReady);
    assert_eq!(deserialized[1].user_id, 200);
}

// ============================================================================
// MODE AND MOD COMBINATIONS
// ============================================================================

#[test]
fn test_all_modes() {
    for mode in [Mode::Standard, Mode::Taiko, Mode::Catch, Mode::Mania] {
        let serialized = mode.serialize();
        let deserialized = Mode::deserialize(&serialized).unwrap();
        assert_eq!(mode, deserialized);
    }
}

#[test]
fn test_complex_mod_combinations() {
    let combinations = vec![
        Mods::None,
        Mods::Hidden | Mods::HardRock,
        Mods::Doubletime | Mods::Flashlight,
        Mods::NoFail | Mods::Easy | Mods::Relax,
        Mods::Perfect | Mods::SuddenDeath,
    ];

    for mods in combinations {
        let serialized = mods.serialize();
        let deserialized = Mods::deserialize(&serialized).unwrap();
        assert_eq!(mods, deserialized);
    }
}

// ============================================================================
// WIN CONDITION FLOWS
// ============================================================================

#[test]
fn test_all_win_conditions() {
    for condition in [
        WinCondition::Score,
        WinCondition::Accuracy,
        WinCondition::Combo,
        WinCondition::ScoreV2,
    ] {
        let serialized = condition.serialize();
        let deserialized = WinCondition::deserialize(&serialized).unwrap();
        assert_eq!(condition, deserialized);
    }
}

// ============================================================================
// TEAM TYPE FLOWS
// ============================================================================

#[test]
fn test_all_team_types() {
    for team_type in [
        MatchTeamType::HeadToHead,
        MatchTeamType::TagCoop,
        MatchTeamType::Vs,
        MatchTeamType::TagVs,
    ] {
        let serialized = team_type.serialize();
        let deserialized = MatchTeamType::deserialize(&serialized).unwrap();
        assert_eq!(team_type, deserialized);
    }
}

// ============================================================================
// PRIVILEGE LEVELS
// ============================================================================

#[test]
fn test_privilege_combinations() {
    let privileges = vec![
        Privileges::Player,
        Privileges::Moderator,
        Privileges::Developer,
        Privileges::Moderator | Privileges::Developer,
    ];

    for priv_level in privileges {
        let serialized = priv_level.serialize();
        let deserialized = Privileges::deserialize(&serialized).unwrap();
        assert_eq!(priv_level, deserialized);
    }
}

// ============================================================================
// BYTE SIZE CONSISTENCY
// ============================================================================

#[test]
fn test_match_byte_size_consistency() {
    let match_obj = Match {
        id: 10,
        in_progress: false,
        powerplay: false,
        mods: Mods::Hidden | Mods::HardRock,
        name: "Size Test",
        password: "pwd",
        beatmap_name: "Beatmap Name",
        beatmap_id: 123,
        beatmap_md5: "hash",
        slots: [MatchSlot::default(); 16],
        host: 5000,
        mode: Mode::Standard,
        win_condition: WinCondition::Score,
        team_type: MatchTeamType::Vs,
        freemod_enabled: false,
        freemods: None,
        random_seed: 1000,
    };

    let byte_size = match_obj.byte_size();
    let serialized = match_obj.serialize();
    assert_eq!(byte_size, serialized.len(), "ByteSized mismatch for Match");
}

#[test]
fn test_irc_message_byte_size_consistency() {
    let msg = IrcMessage {
        sender: "testuser",
        text: "test",
        recipient: "#test",
        sender_id: 1000,
    };

    let byte_size = msg.byte_size();
    let serialized = msg.serialize();
    assert_eq!(byte_size, serialized.len(), "ByteSized mismatch for IrcMessage");
}

// ============================================================================
// MODS ROUNDTRIPS
// ============================================================================

#[test]
fn test_mods_roundtrip() {
    let test_values = vec![
        Mods::None,
        Mods::NoFail,
        Mods::Easy | Mods::Hidden,
        Mods::HardRock | Mods::Flashlight,
        Mods::Doubletime | Mods::Nightcore,
    ];

    for mods in test_values {
        let serialized = mods.serialize();
        let deserialized = Mods::deserialize(&serialized).unwrap();
        assert_eq!(mods, deserialized);
        assert_eq!(mods.bits(), deserialized.bits());
    }
}

// ============================================================================
// REPLAY FRAME ROUNDTRIPS
// ============================================================================

#[test]
fn test_replay_frame_roundtrip() {
    let frame = ReplayFrame {
        button_state: ButtonState::M1 | ButtonState::K2,
        taiko_byte: 5,
        x: 100.5,
        y: 200.75,
        time: 5000,
    };

    let serialized = frame.serialize();
    let deserialized = ReplayFrame::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.button_state, ButtonState::M1 | ButtonState::K2);
    assert_eq!(deserialized.taiko_byte, 5);
    assert_eq!(deserialized.x, 100.5);
    assert_eq!(deserialized.y, 200.75);
    assert_eq!(deserialized.time, 5000);
}
