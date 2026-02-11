use crate::serde::uleb128::v32;
use std::io::Error;

pub struct BinaryReader<'a> {
    stream: &'a [u8],
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub const fn from(stream: &'a [u8]) -> Self {
        Self {
            stream,
            position: 0,
        }
    }

    pub const fn reset(&mut self) {
        self.position = 0;
    }

    pub const fn pos(&self) -> usize {
        self.position
    }

    pub const fn skip(&mut self, count: usize) {
        self.position += count;
    }

    pub const fn can_read(&self) -> bool {
        self.stream.len() > self.position
    }

    pub const fn can_read_n(&self, count: usize) -> bool {
        self.stream.len() > self.position + count - 1
    }

    pub fn next(&mut self) -> std::io::Result<u8> {
        if !self.can_read() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "unexpected end of stream",
            ));
        }

        let r = self.stream[self.position];
        self.position += 1;
        Ok(r)
    }

    pub fn next_range(&mut self, count: usize) -> std::io::Result<&'a [u8]> {
        if !self.can_read_n(count) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "unexpected end of stream",
            ));
        }
        let end = self.position + count;
        let r = &self.stream[self.position..end];
        self.position = end;
        Ok(r)
    }

    pub fn next_range_const<const N: usize>(&mut self) -> std::io::Result<[u8; N]> {
        if !self.can_read_n(N) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "unexpected end of stream",
            ));
        }

        let end = self.position + N;
        let r = self.stream[self.position..].first_chunk().unwrap();
        self.position = end;
        Ok(*r)
    }
}

pub trait BinaryDeserialize<'a>: Sized {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>;

    fn deserialize(data: &'a [u8]) -> std::io::Result<Self> {
        let mut reader = BinaryReader::from(data);
        Self::read_from(&mut reader)
    }
}

macro_rules! impl_deserialize {
    ($t:ty) => {
        impl<'a> BinaryDeserialize<'a> for $t {
            fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
                let bytes = reader.next_range_const::<{ std::mem::size_of::<$t>() }>()?;
                Ok(<$t>::from_le_bytes(bytes))
            }
        }
    };
    ($t:ty, $($tt:ty), +) => {
        impl_deserialize!($t);
        impl_deserialize!($($tt), +);
    }
}

impl BinaryDeserialize<'_> for () {
    fn read_from(_reader: &mut BinaryReader<'_>) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> BinaryDeserialize<'a> for u8 {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        reader.next()
    }
}

impl<'a> BinaryDeserialize<'a> for bool {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        Ok(reader.next()? == 1)
    }
}

impl_deserialize!(u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl<'a> BinaryDeserialize<'a> for &'a str {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let osu_type = reader.next()?;
        if osu_type != 0x0b {
            return Ok(Self::default());
        }

        let len = v32::read_from(reader)?;
        let bytes = reader.next_range(len.0 as usize)?;
        std::str::from_utf8(bytes)
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))
    }
}

impl<'a> BinaryDeserialize<'a> for String {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        Ok(<&'a str>::read_from(reader)?.to_string())
    }
}

impl<'a, const N: usize, T: BinaryDeserialize<'a> + Copy> BinaryDeserialize<'a> for [T; N] {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let init = T::read_from(reader)?;
        let mut r = [init; N];
        for i in 1..N {
            r[i] = T::read_from(reader)?;
        }
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BinaryReader basic operations
    #[test]
    fn test_reader_from_and_pos() {
        let data = [1u8, 2, 3];
        let reader = BinaryReader::from(&data);
        assert_eq!(reader.pos(), 0);
    }

    #[test]
    fn test_reader_reset() {
        let data = [1u8, 2, 3];
        let mut reader = BinaryReader::from(&data);
        reader.next().unwrap();
        assert_eq!(reader.pos(), 1);
        reader.reset();
        assert_eq!(reader.pos(), 0);
    }

    #[test]
    fn test_reader_skip() {
        let data = [1u8, 2, 3];
        let mut reader = BinaryReader::from(&data);
        reader.skip(2);
        assert_eq!(reader.pos(), 2);
        assert_eq!(reader.next().unwrap(), 3);
    }

    #[test]
    fn test_reader_can_read_empty() {
        let data: [u8; 0] = [];
        let reader = BinaryReader::from(data.as_slice());
        assert!(!reader.can_read());
    }

    #[test]
    fn test_reader_can_read_nonempty() {
        let data = [1u8];
        let reader = BinaryReader::from(&data);
        assert!(reader.can_read());
    }

    #[test]
    fn test_reader_can_read_n() {
        let data = [1u8, 2, 3];
        let reader = BinaryReader::from(&data);
        assert!(reader.can_read_n(3));
        assert!(!reader.can_read_n(4));
    }

    #[test]
    fn test_reader_next_eof() {
        let data: [u8; 0] = [];
        let mut reader = BinaryReader::from(data.as_slice());
        let result = reader.next();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_reader_next_range_eof() {
        let data = [1u8, 2];
        let mut reader = BinaryReader::from(&data);
        let result = reader.next_range(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_reader_next_range_const_eof() {
        let data = [1u8];
        let mut reader = BinaryReader::from(&data);
        let result = reader.next_range_const::<4>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
    }

    // Unit type deserialization
    #[test]
    fn test_deserialize_unit() {
        let data: [u8; 0] = [];
        let result = <()>::deserialize(&data);
        assert!(result.is_ok());
    }

    // u8 deserialization
    #[test]
    fn test_deserialize_u8() {
        let data = [42u8];
        assert_eq!(u8::deserialize(&data).unwrap(), 42);
    }

    #[test]
    fn test_deserialize_u8_eof() {
        let data: [u8; 0] = [];
        assert!(u8::deserialize(&data).is_err());
    }

    // bool deserialization
    #[test]
    fn test_deserialize_bool_true() {
        let data = [1u8];
        assert!(bool::deserialize(&data).unwrap());
    }

    #[test]
    fn test_deserialize_bool_false() {
        let data = [0u8];
        assert!(!bool::deserialize(&data).unwrap());
    }

    #[test]
    fn test_deserialize_bool_nonone_is_false() {
        // Values other than 1 should be false
        let data = [2u8];
        assert!(!bool::deserialize(&data).unwrap());
    }

    #[test]
    fn test_deserialize_bool_eof() {
        let data: [u8; 0] = [];
        assert!(bool::deserialize(&data).is_err());
    }

    // u16 deserialization
    #[test]
    fn test_deserialize_u16() {
        let data = 1234u16.to_le_bytes();
        assert_eq!(u16::deserialize(&data).unwrap(), 1234);
    }

    #[test]
    fn test_deserialize_u16_eof() {
        let data = [0u8]; // only 1 byte, need 2
        assert!(u16::deserialize(&data).is_err());
    }

    // u32 deserialization
    #[test]
    fn test_deserialize_u32() {
        let data = 123456u32.to_le_bytes();
        assert_eq!(u32::deserialize(&data).unwrap(), 123456);
    }

    #[test]
    fn test_deserialize_u32_eof() {
        let data = [0u8, 1, 2]; // only 3 bytes, need 4
        assert!(u32::deserialize(&data).is_err());
    }

    // u64 deserialization
    #[test]
    fn test_deserialize_u64() {
        let data = 9999999999u64.to_le_bytes();
        assert_eq!(u64::deserialize(&data).unwrap(), 9999999999);
    }

    #[test]
    fn test_deserialize_u64_eof() {
        let data = [0u8; 7]; // only 7 bytes, need 8
        assert!(u64::deserialize(&data).is_err());
    }

    // i8 deserialization
    #[test]
    fn test_deserialize_i8() {
        let data = (-42i8).to_le_bytes();
        assert_eq!(i8::deserialize(&data).unwrap(), -42);
    }

    #[test]
    fn test_deserialize_i8_eof() {
        let data: [u8; 0] = [];
        assert!(i8::deserialize(&data).is_err());
    }

    // i16 deserialization
    #[test]
    fn test_deserialize_i16() {
        let data = (-1234i16).to_le_bytes();
        assert_eq!(i16::deserialize(&data).unwrap(), -1234);
    }

    #[test]
    fn test_deserialize_i16_eof() {
        let data = [0u8]; // only 1 byte, need 2
        assert!(i16::deserialize(&data).is_err());
    }

    // i32 deserialization
    #[test]
    fn test_deserialize_i32() {
        let data = (-100000i32).to_le_bytes();
        assert_eq!(i32::deserialize(&data).unwrap(), -100000);
    }

    #[test]
    fn test_deserialize_i32_eof() {
        let data = [0u8; 3]; // only 3 bytes, need 4
        assert!(i32::deserialize(&data).is_err());
    }

    // i64 deserialization
    #[test]
    fn test_deserialize_i64() {
        let data = (-9999999999i64).to_le_bytes();
        assert_eq!(i64::deserialize(&data).unwrap(), -9999999999);
    }

    #[test]
    fn test_deserialize_i64_eof() {
        let data = [0u8; 7]; // only 7 bytes, need 8
        assert!(i64::deserialize(&data).is_err());
    }

    // f32 deserialization
    #[test]
    fn test_deserialize_f32() {
        let data = 3.14f32.to_le_bytes();
        assert_eq!(f32::deserialize(&data).unwrap(), 3.14);
    }

    #[test]
    fn test_deserialize_f32_eof() {
        let data = [0u8; 3]; // only 3 bytes, need 4
        assert!(f32::deserialize(&data).is_err());
    }

    // f64 deserialization
    #[test]
    fn test_deserialize_f64() {
        let data = 3.14159265f64.to_le_bytes();
        assert_eq!(f64::deserialize(&data).unwrap(), 3.14159265);
    }

    #[test]
    fn test_deserialize_f64_eof() {
        let data = [0u8; 7]; // only 7 bytes, need 8
        assert!(f64::deserialize(&data).is_err());
    }

    // &str deserialization
    #[test]
    fn test_deserialize_str_with_0x0b_prefix() {
        // 0x0b prefix, then uleb128 length (5), then "hello"
        let data = [0x0b, 0x05, b'h', b'e', b'l', b'l', b'o'];
        let result = <&str>::deserialize(&data).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_deserialize_str_without_0x0b_prefix() {
        // Non-0x0b prefix returns empty string
        let data = [0x00];
        let result = <&str>::deserialize(&data).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_deserialize_str_eof_on_prefix() {
        let data: [u8; 0] = [];
        assert!(<&str>::deserialize(&data).is_err());
    }

    #[test]
    fn test_deserialize_str_invalid_utf8() {
        // 0x0b prefix, length 2, then invalid UTF-8 bytes
        let data = [0x0b, 0x02, 0xFF, 0xFE];
        let result = <&str>::deserialize(&data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_deserialize_str_empty_string() {
        // 0x0b prefix, length 0
        let data = [0x0b, 0x00];
        let result = <&str>::deserialize(&data).unwrap();
        assert_eq!(result, "");
    }

    // String deserialization
    #[test]
    fn test_deserialize_string_with_prefix() {
        let data = [0x0b, 0x03, b'a', b'b', b'c'];
        let result = String::deserialize(&data).unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_deserialize_string_without_prefix() {
        let data = [0x00];
        let result = String::deserialize(&data).unwrap();
        assert_eq!(result, "");
    }

    // Array deserialization
    #[test]
    fn test_deserialize_array_u8() {
        let data = [10u8, 20, 30];
        let result = <[u8; 3]>::deserialize(&data).unwrap();
        assert_eq!(result, [10, 20, 30]);
    }

    #[test]
    fn test_deserialize_array_u16() {
        let mut data = Vec::new();
        data.extend_from_slice(&100u16.to_le_bytes());
        data.extend_from_slice(&200u16.to_le_bytes());
        let result = <[u16; 2]>::deserialize(&data).unwrap();
        assert_eq!(result, [100, 200]);
    }

    #[test]
    fn test_deserialize_array_eof() {
        let data = [1u8, 2]; // only 2 bytes, need 3 for [u8; 3]
        assert!(<[u8; 3]>::deserialize(&data).is_err());
    }

    // Deserialize trait default method
    #[test]
    fn test_deserialize_trait_default_method() {
        let data = 42u32.to_le_bytes();
        let result = u32::deserialize(&data).unwrap();
        assert_eq!(result, 42);
    }

    // Sequential reads from a single reader
    #[test]
    fn test_reader_sequential_reads() {
        let mut data = Vec::new();
        data.extend_from_slice(&42u16.to_le_bytes());
        data.extend_from_slice(&100u32.to_le_bytes());
        data.push(7u8);

        let mut reader = BinaryReader::from(&data);
        assert_eq!(u16::read_from(&mut reader).unwrap(), 42);
        assert_eq!(u32::read_from(&mut reader).unwrap(), 100);
        assert_eq!(u8::read_from(&mut reader).unwrap(), 7);
        assert!(!reader.can_read());
    }

    #[test]
    fn test_reader_next_range_valid() {
        let data = [1u8, 2, 3, 4, 5];
        let mut reader = BinaryReader::from(&data);
        let slice = reader.next_range(3).unwrap();
        assert_eq!(slice, &[1, 2, 3]);
        assert_eq!(reader.pos(), 3);
    }

    #[test]
    fn test_reader_next_range_const_valid() {
        let data = [10u8, 20, 30, 40];
        let mut reader = BinaryReader::from(&data);
        let arr = reader.next_range_const::<4>().unwrap();
        assert_eq!(arr, [10, 20, 30, 40]);
    }
}
