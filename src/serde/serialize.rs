use crate::serde::byte_sized::ByteSized;
use crate::serde::uleb128::v32;

pub struct BinaryWriter {
    pub data: Vec<u8>,
}

impl BinaryWriter {
    pub fn with_length(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }

    pub fn write_byte(&mut self, v: u8) {
        self.data.push(v);
    }

    pub fn write_byte_slice(&mut self, v: &[u8]) {
        self.data.extend_from_slice(v);
    }

    pub fn write_const<const SIZE: usize>(&mut self, array: [u8; SIZE]) {
        self.data.extend_from_slice(&array);
    }

    pub fn write<T: BinarySerialize>(&mut self, v: T) {
        v.write_to(self);
    }

    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

pub trait BinarySerialize: ByteSized {
    fn write_to(&self, writer: &mut BinaryWriter);

    fn serialize(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut writer = BinaryWriter::with_length(self.byte_size());
        self.write_to(&mut writer);
        writer.data()
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
