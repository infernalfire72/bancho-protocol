use crate::serde::byte_sized::ByteSized;
use crate::serde::uleb128::v32;

pub struct BinaryWriter<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}

impl<'a> BinaryWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, offset: 0 }
    }

    pub fn write_byte(&mut self, v: u8) {
        self.buffer[self.offset] = v;
        self.offset += 1;
    }

    pub fn write_byte_slice(&mut self, slice: &[u8]) {
        self.buffer[self.offset..self.offset + slice.len()].copy_from_slice(slice);
        self.offset += slice.len();
    }

    pub fn write_const<const SIZE: usize>(&mut self, array: [u8; SIZE]) {
        self.buffer[self.offset..self.offset + SIZE].copy_from_slice(&array);
        self.offset += SIZE;
    }

    pub fn write<T: BinarySerialize>(&mut self, v: T) {
        v.write_to(self);
    }
}

pub trait BinarySerialize: ByteSized {
    fn write_to(&self, writer: &mut BinaryWriter);

    fn serialize(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut buffer = vec![0; self.byte_size()];
        let mut writer = BinaryWriter::new(&mut buffer);
        self.write_to(&mut writer);
        buffer
    }
}

macro_rules! impl_serialize {
    ($t:ty) => {
        impl BinarySerialize for $t {
            fn write_to(&self, writer: &mut BinaryWriter) {
                writer.write_const(self.to_le_bytes())
            }
        }
    };
    ($t:ty, $($tt:ty), +) => {
        impl_serialize!($t);
        impl_serialize!($($tt), +);
    }
}

impl_serialize!(u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl BinarySerialize for () {
    fn write_to(&self, _writer: &mut BinaryWriter) {}
}

impl BinarySerialize for u8 {
    fn write_to(&self, writer: &mut BinaryWriter) {
        writer.write_byte(*self);
    }
}

impl BinarySerialize for bool {
    fn write_to(&self, writer: &mut BinaryWriter) {
        u8::write_to(
            &match self {
                true => 1,
                _ => 0,
            },
            writer,
        )
    }
}

impl BinarySerialize for str {
    fn write_to(&self, writer: &mut BinaryWriter) {
        if self.is_empty() {
            writer.write_byte(0);
            return;
        }

        writer.write_byte(0x0b);
        v32(self.len() as u32).write_to(writer);
        writer.write_byte_slice(self.as_bytes());
    }
}

impl BinarySerialize for String {
    fn write_to(&self, writer: &mut BinaryWriter) {
        str::write_to(self, writer)
    }
}

impl<const N: usize, T: BinarySerialize> BinarySerialize for [T; N] {
    fn write_to(&self, writer: &mut BinaryWriter) {
        for i in self {
            BinarySerialize::write_to(i, writer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::deserialize::BinaryDeserialize;

    // Primitive roundtrips via serialize()
    #[test]
    fn test_serialize_u8() {
        let val = 42u8;
        let bytes = val.serialize();
        assert_eq!(bytes, vec![42]);
    }

    #[test]
    fn test_serialize_u16() {
        let val = 1234u16;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_u32() {
        let val = 123456u32;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_u64() {
        let val = 9999999999u64;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_i8() {
        let val = -42i8;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_i16() {
        let val = -1234i16;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_i32() {
        let val = -100000i32;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_i64() {
        let val = -9999999999i64;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_f32() {
        let val = 3.14f32;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    #[test]
    fn test_serialize_f64() {
        let val = 3.14159265f64;
        let bytes = val.serialize();
        assert_eq!(bytes, val.to_le_bytes().to_vec());
    }

    // bool serialization
    #[test]
    fn test_serialize_bool_true() {
        let bytes = true.serialize();
        assert_eq!(bytes, vec![1]);
    }

    #[test]
    fn test_serialize_bool_false() {
        let bytes = false.serialize();
        assert_eq!(bytes, vec![0]);
    }

    // Unit serialization
    #[test]
    fn test_serialize_unit() {
        let bytes = ().serialize();
        assert!(bytes.is_empty());
    }

    // str serialization (via write_to, since str is unsized)
    #[test]
    fn test_serialize_str_empty() {
        let mut writer = BinaryWriter::with_length(1);
        "".write_to(&mut writer);
        assert_eq!(writer.data(), vec![0x00]);
    }

    #[test]
    fn test_serialize_str_nonempty() {
        let mut writer = BinaryWriter::with_length(4);
        "hi".write_to(&mut writer);
        // 0x0b prefix, uleb128 length (2), then "hi"
        assert_eq!(writer.data(), vec![0x0b, 0x02, b'h', b'i']);
    }

    // String serialization
    #[test]
    fn test_serialize_string() {
        let s = String::from("abc");
        let bytes = s.serialize();
        assert_eq!(bytes, vec![0x0b, 0x03, b'a', b'b', b'c']);
    }

    // Array serialization
    #[test]
    fn test_serialize_array_u8() {
        let arr: [u8; 3] = [10, 20, 30];
        let bytes = arr.serialize();
        assert_eq!(bytes, vec![10, 20, 30]);
    }

    #[test]
    fn test_serialize_array_u16() {
        let arr: [u16; 2] = [100, 200];
        let bytes = arr.serialize();
        let mut expected = Vec::new();
        expected.extend_from_slice(&100u16.to_le_bytes());
        expected.extend_from_slice(&200u16.to_le_bytes());
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_serialize_array_roundtrip() {
        let arr: [u32; 3] = [1, 2, 3];
        let bytes = arr.serialize();
        let decoded = <[u32; 3]>::deserialize(&bytes).unwrap();
        assert_eq!(decoded, arr);
    }

    // BinaryWriter methods
    #[test]
    fn test_binary_writer_write_byte_slice() {
        let mut writer = BinaryWriter::with_length(4);
        writer.write_byte_slice(&[1, 2, 3, 4]);
        assert_eq!(writer.data(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_binary_writer_write_const() {
        let mut writer = BinaryWriter::with_length(3);
        writer.write_const([10, 20, 30]);
        assert_eq!(writer.data(), vec![10, 20, 30]);
    }

    #[test]
    fn test_binary_writer_write_generic() {
        let mut writer = BinaryWriter::with_length(4);
        writer.write(42u32);
        assert_eq!(writer.data(), 42u32.to_le_bytes().to_vec());
    }

    // serialize() trait default method (preallocates correct size)
    #[test]
    fn test_serialize_trait_default_method() {
        let val = 42u32;
        let bytes = val.serialize();
        assert_eq!(bytes.len(), 4);
        assert_eq!(u32::deserialize(&bytes).unwrap(), 42);
    }
}
