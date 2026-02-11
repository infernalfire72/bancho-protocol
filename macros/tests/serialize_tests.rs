use bancho_protocol_macros::{BinarySerialize, ByteSized};
use bancho_protocol::serde::BinarySerialize as _;
use bancho_protocol::serde::byte_sized::ByteSized as _;

#[test]
fn test_simple_struct_serialize() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct TestStruct {
        a: u32,
        b: u16,
    }

    let s = TestStruct { a: 42, b: 100 };
    let bytes = s.serialize();

    // u32 (4 bytes) + u16 (2 bytes) = 6 bytes
    assert_eq!(bytes.len(), 6);

    // Verify values are correctly serialized in little endian
    assert_eq!(&bytes[0..4], 42u32.to_le_bytes());
    assert_eq!(&bytes[4..6], 100u16.to_le_bytes());
}

#[test]
fn test_single_field_struct_serialize() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct SingleField {
        value: u8,
    }

    let s = SingleField { value: 255 };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 255);
}

#[test]
fn test_multiple_u8_fields() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct MultiU8 {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    let s = MultiU8 { a: 1, b: 2, c: 3, d: 4 };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes, vec![1, 2, 3, 4]);
}

#[test]
fn test_i32_serialization() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct SignedValue {
        positive: i32,
        negative: i32,
    }

    let s = SignedValue { positive: 42, negative: -42 };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[0..4], 42i32.to_le_bytes());
    assert_eq!(&bytes[4..8], (-42i32).to_le_bytes());
}

#[test]
fn test_u64_serialization() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct LargeValue {
        big: u64,
    }

    let s = LargeValue { big: 0x0102030405060708u64 };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes.as_slice(), 0x0102030405060708u64.to_le_bytes());
}

#[test]
fn test_bool_serialization() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct BoolValue {
        is_true: bool,
        is_false: bool,
    }

    let s = BoolValue { is_true: true, is_false: false };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[1], 0);
}

#[test]
fn test_zero_values() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct ZeroStruct {
        a: u32,
        b: u16,
        c: u8,
    }

    let s = ZeroStruct { a: 0, b: 0, c: 0 };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 7);
    assert!(bytes.iter().all(|&b| b == 0));
}

#[test]
fn test_max_values() {
    #[derive(BinarySerialize, ByteSized)]
    #[crate_root(bancho_protocol)]
    struct MaxStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    let s = MaxStruct { a: u8::MAX, b: u16::MAX, c: u32::MAX };
    let bytes = s.serialize();

    assert_eq!(bytes.len(), 7);
}
