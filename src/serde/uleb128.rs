use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use std::fmt::{Debug, Display, Formatter};

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct v32(pub u32);

impl Display for v32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for v32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl ByteSized for v32 {
    fn byte_size(&self) -> usize {
        // (f64::floor(f64::log2(self.0 as _) / 7.0) + 1.0) as _
        if (self.0 & 0xFFFFFF80) == 0 {
            1
        } else if (self.0 & 0xFFFFC000) == 0 {
            2
        } else if (self.0 & 0xFFE00000) == 0 {
            3
        } else if (self.0 & 0xF0000000) == 0 {
            4
        } else {
            5
        }
    }
}

impl BinarySerialize for v32 {
    fn write_to(&self, writer: &mut BinaryWriter) {
        let mut v = self.0;
        loop {
            let mut b: u8 = (v & 0x7f) as _;
            v >>= 7;
            if v == 0 {
                writer.write_byte(b);
                return;
            } else {
                b |= 0x80;
                writer.write_byte(b);
            }
        }
    }
}

impl<'a> BinaryDeserialize<'a> for v32 {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let mut r = 0;
        let mut s = 0;
        loop {
            let b = reader.next()?;
            r |= ((b & 0x7f) as u32) << s;
            if (b & 0x80) == 0 {
                break;
            }

            s += 7;
        }

        Ok(Self(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Display impl
    #[test]
    fn test_v32_display() {
        let v = v32(42);
        assert_eq!(format!("{}", v), "42");
    }

    #[test]
    fn test_v32_display_zero() {
        let v = v32(0);
        assert_eq!(format!("{}", v), "0");
    }

    #[test]
    fn test_v32_display_large() {
        let v = v32(1_000_000);
        assert_eq!(format!("{}", v), "1000000");
    }

    // Debug impl
    #[test]
    fn test_v32_debug() {
        let v = v32(42);
        assert_eq!(format!("{:?}", v), "42");
    }

    #[test]
    fn test_v32_debug_zero() {
        let v = v32(0);
        assert_eq!(format!("{:?}", v), "0");
    }

    // ByteSized - 1 byte range (0..128)
    #[test]
    fn test_v32_byte_size_1_byte() {
        assert_eq!(v32(0).byte_size(), 1);
        assert_eq!(v32(1).byte_size(), 1);
        assert_eq!(v32(127).byte_size(), 1);
    }

    // ByteSized - 2 byte range (128..16384)
    #[test]
    fn test_v32_byte_size_2_bytes() {
        assert_eq!(v32(128).byte_size(), 2);
        assert_eq!(v32(16383).byte_size(), 2);
    }

    // ByteSized - 3 byte range (16384..2097152)
    #[test]
    fn test_v32_byte_size_3_bytes() {
        assert_eq!(v32(16384).byte_size(), 3);
        assert_eq!(v32(2097151).byte_size(), 3);
    }

    // ByteSized - 4 byte range (2097152..268435456)
    #[test]
    fn test_v32_byte_size_4_bytes() {
        assert_eq!(v32(2097152).byte_size(), 4);
        assert_eq!(v32(268435455).byte_size(), 4);
    }

    // ByteSized - 5 byte range (268435456..)
    #[test]
    fn test_v32_byte_size_5_bytes() {
        assert_eq!(v32(268435456).byte_size(), 5);
        assert_eq!(v32(u32::MAX).byte_size(), 5);
    }

    // write_to (BinarySerialize)
    #[test]
    fn test_v32_write_to_single_byte() {
        let v = v32(42);
        let bytes = v.serialize();
        assert_eq!(bytes, vec![42]);
    }

    #[test]
    fn test_v32_write_to_zero() {
        let v = v32(0);
        let bytes = v.serialize();
        assert_eq!(bytes, vec![0]);
    }

    #[test]
    fn test_v32_write_to_multi_byte() {
        let v = v32(300);
        let bytes = v.serialize();
        // 300 = 0b100101100 -> 0b0101100 | 0x80, 0b0000010
        assert_eq!(bytes, vec![0xAC, 0x02]);
    }

    #[test]
    fn test_v32_write_to_max_single_byte() {
        let v = v32(127);
        let bytes = v.serialize();
        assert_eq!(bytes, vec![127]);
    }

    #[test]
    fn test_v32_write_to_min_two_bytes() {
        let v = v32(128);
        let bytes = v.serialize();
        assert_eq!(bytes, vec![0x80, 0x01]);
    }

    // Roundtrip serialize/deserialize
    #[test]
    fn test_v32_roundtrip_zero() {
        let original = v32(0);
        let bytes = original.serialize();
        let decoded = v32::deserialize(&bytes).unwrap();
        assert_eq!(decoded.0, 0);
    }

    #[test]
    fn test_v32_roundtrip_small() {
        let original = v32(42);
        let bytes = original.serialize();
        let decoded = v32::deserialize(&bytes).unwrap();
        assert_eq!(decoded.0, 42);
    }

    #[test]
    fn test_v32_roundtrip_large() {
        let original = v32(1_000_000);
        let bytes = original.serialize();
        let decoded = v32::deserialize(&bytes).unwrap();
        assert_eq!(decoded.0, 1_000_000);
    }

    #[test]
    fn test_v32_roundtrip_max() {
        let original = v32(u32::MAX);
        let bytes = original.serialize();
        let decoded = v32::deserialize(&bytes).unwrap();
        assert_eq!(decoded.0, u32::MAX);
    }

    // Deserialize error
    #[test]
    fn test_v32_deserialize_eof() {
        let data: [u8; 0] = [];
        assert!(v32::deserialize(&data).is_err());
    }

    #[test]
    fn test_v32_deserialize_truncated_multibyte() {
        // A continuation byte (0x80 set) with no following byte
        let data = [0x80];
        assert!(v32::deserialize(&data).is_err());
    }

    // Serialized length matches byte_size
    #[test]
    fn test_v32_serialize_length_matches_byte_size() {
        for val in [0, 1, 127, 128, 16383, 16384, 2097151, 2097152, 268435455, 268435456, u32::MAX] {
            let v = v32(val);
            assert_eq!(v.serialize().len(), v.byte_size(), "mismatch for value {}", val);
        }
    }
}
