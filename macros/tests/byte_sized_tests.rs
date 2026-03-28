use bancho_protocol_macros::ByteSized;
use bancho_protocol::serde::byte_sized::ByteSized as _;

#[test]
fn test_simple_struct_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct TestStruct {
        a: u32,
        b: u16,
    }

    let s = TestStruct { a: 42, b: 100 };
    // u32 (4 bytes) + u16 (2 bytes) = 6 bytes
    assert_eq!(s.byte_size(), 6);
}

#[test]
fn test_single_field_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct SingleField {
        value: u8,
    }

    let s = SingleField { value: 255 };
    assert_eq!(s.byte_size(), 1);
}

#[test]
fn test_four_u8_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct MultiU8 {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    let s = MultiU8 { a: 1, b: 2, c: 3, d: 4 };
    assert_eq!(s.byte_size(), 4);
}

#[test]
fn test_i32_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct SignedValue {
        positive: i32,
        negative: i32,
    }

    let s = SignedValue { positive: 42, negative: -42 };
    // i32 (4 bytes) + i32 (4 bytes) = 8 bytes
    assert_eq!(s.byte_size(), 8);
}

#[test]
fn test_u64_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct LargeValue {
        big: u64,
    }

    let s = LargeValue { big: 0x0102030405060708 };
    assert_eq!(s.byte_size(), 8);
}

#[test]
fn test_bool_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct BoolValue {
        is_true: bool,
        is_false: bool,
    }

    let s = BoolValue { is_true: true, is_false: false };
    // bool (1 byte) + bool (1 byte) = 2 bytes
    assert_eq!(s.byte_size(), 2);
}

#[test]
fn test_mixed_types_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct MixedStruct {
        a: u8,      // 1
        b: u16,     // 2
        c: u32,     // 4
        d: u64,     // 8
    }

    let s = MixedStruct { a: 1, b: 2, c: 3, d: 4 };
    // 1 + 2 + 4 + 8 = 15 bytes
    assert_eq!(s.byte_size(), 15);
}

#[test]
fn test_byte_size_zero_values() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct ZeroStruct {
        a: u32,
        b: u16,
        c: u8,
    }

    let s = ZeroStruct { a: 0, b: 0, c: 0 };
    // Byte size doesn't depend on values
    assert_eq!(s.byte_size(), 7);
}

#[test]
fn test_byte_size_max_values() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct MaxStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    let s = MaxStruct { a: u8::MAX, b: u16::MAX, c: u32::MAX };
    // Byte size doesn't depend on values
    assert_eq!(s.byte_size(), 7);
}

#[test]
fn test_byte_size_independence_from_value() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct TestStruct {
        a: u32,
    }

    let s1 = TestStruct { a: 0 };
    let s2 = TestStruct { a: u32::MAX };
    let s3 = TestStruct { a: 42 };

    assert_eq!(s1.byte_size(), s2.byte_size());
    assert_eq!(s2.byte_size(), s3.byte_size());
    assert_eq!(s1.byte_size(), 4);
}

#[test]
fn test_large_struct_byte_size() {
    #[derive(ByteSized)]
    #[crate_root(bancho_protocol)]
    struct LargeStruct {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u32,
    }

    let s = LargeStruct { a: 1, b: 2, c: 3, d: 4, e: 5 };
    // 4 * 8 + 4 = 36 bytes
    assert_eq!(s.byte_size(), 36);
}
