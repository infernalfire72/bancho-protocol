use bancho_protocol_macros::BinaryDeserialize;
use bancho_protocol::serde::deserialize::BinaryDeserialize as _;

#[test]
fn test_simple_struct_deserialize() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct TestStruct {
        a: u32,
        b: u16,
    }

    let mut bytes = 42u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&100u16.to_le_bytes());

    let s = TestStruct::deserialize(&bytes).unwrap();
    assert_eq!(s.a, 42);
    assert_eq!(s.b, 100);
}

#[test]
fn test_single_field_struct_deserialize() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct SingleField {
        value: u8,
    }

    let bytes = vec![255];
    let s = SingleField::deserialize(&bytes).unwrap();
    assert_eq!(s.value, 255);
}

#[test]
fn test_multiple_u8_fields_deserialize() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct MultiU8 {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    let bytes = vec![1, 2, 3, 4];
    let s = MultiU8::deserialize(&bytes).unwrap();
    assert_eq!(s.a, 1);
    assert_eq!(s.b, 2);
    assert_eq!(s.c, 3);
    assert_eq!(s.d, 4);
}

#[test]
fn test_i32_deserialization() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct SignedValue {
        positive: i32,
        negative: i32,
    }

    let mut bytes = 42i32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&(-42i32).to_le_bytes());

    let s = SignedValue::deserialize(&bytes).unwrap();
    assert_eq!(s.positive, 42);
    assert_eq!(s.negative, -42);
}

#[test]
fn test_u64_deserialization() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct LargeValue {
        big: u64,
    }

    let bytes = 0x0102030405060708u64.to_le_bytes().to_vec();
    let s = LargeValue::deserialize(&bytes).unwrap();
    assert_eq!(s.big, 0x0102030405060708);
}

#[test]
fn test_bool_deserialization_true() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct BoolValue {
        flag: bool,
    }

    let bytes = vec![1];
    let s = BoolValue::deserialize(&bytes).unwrap();
    assert!(s.flag);
}

#[test]
fn test_bool_deserialization_false() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct BoolValue {
        flag: bool,
    }

    let bytes = vec![0];
    let s = BoolValue::deserialize(&bytes).unwrap();
    assert!(!s.flag);
}

#[test]
fn test_insufficient_bytes() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct TestStruct {
        a: u32,
        b: u16,
    }

    let bytes = vec![1, 2, 3]; // Only 3 bytes, need 6
    let result = TestStruct::deserialize(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_empty_bytes() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct SingleField {
        value: u8,
    }

    let bytes = vec![];
    let result = SingleField::deserialize(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_zero_values_deserialize() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct ZeroStruct {
        a: u32,
        b: u16,
        c: u8,
    }

    let bytes = vec![0, 0, 0, 0, 0, 0, 0];
    let s = ZeroStruct::deserialize(&bytes).unwrap();
    assert_eq!(s.a, 0);
    assert_eq!(s.b, 0);
    assert_eq!(s.c, 0);
}

#[test]
fn test_max_values_deserialize() {
    #[derive(BinaryDeserialize)]
    #[crate_root(bancho_protocol)]
    struct MaxStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    let mut bytes = vec![u8::MAX];
    bytes.extend_from_slice(&u16::MAX.to_le_bytes());
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());

    let s = MaxStruct::deserialize(&bytes).unwrap();
    assert_eq!(s.a, u8::MAX);
    assert_eq!(s.b, u16::MAX);
    assert_eq!(s.c, u32::MAX);
}
