use crate::serde::byte_sized::ByteSized;
use crate::serde::uleb128::v32;

pub struct BinaryWriter {
    pub(crate) underlying: Vec<u8>,
    pub(crate) position: usize,
}

impl BinaryWriter {
    pub fn with_length(cap: usize) -> Self {
        Self {
            underlying: vec![0; cap],
            position: 0,
        }
    }

    pub fn skip(&mut self, count: usize) {
        self.position += count
    }

    pub fn write_byte(&mut self, v: u8) {
        self.underlying[self.position] = v;
        self.position += 1
    }

    pub fn write_byte_slice(&mut self, v: &[u8]) {
        unsafe { std::ptr::copy(
            v.as_ptr(),
            self.underlying.as_mut_ptr().offset(self.position as _),
            v.len()
        ) }
        self.position += v.len()
    }

    pub fn write<T: BinarySerialize>(&mut self, v: T) {
        v.write_to(self);
    }

    pub fn data(self) -> Vec<u8> {
        self.underlying
    }
}

pub trait BinarySerialize: ByteSized {
    fn write_to(&self, writer: &mut BinaryWriter);

    fn serialize(&self) -> Vec<u8> where Self: Sized {
        let mut writer = BinaryWriter::with_length(self.byte_size());
        self.write_to(&mut writer);
        writer.data()
    }
}

macro_rules! impl_serialize {
    ($t:ty) => {
        impl BinarySerialize for $t {
            fn write_to(&self, writer: &mut BinaryWriter) {
                unsafe { std::ptr::write_unaligned(
                    writer.underlying.as_mut_ptr().offset(writer.position as _) as *mut $t,
                    *self) };
                writer.position += std::mem::size_of::<$t>()
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
        u8::write_to(&match self { true => 1, _ => 0 }, writer)
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

use tuple_list::TupleList;

impl BinarySerialize for () {
    fn write_to(&self, _writer: &mut BinaryWriter) {}
}

impl<Head: BinarySerialize, Tail: BinarySerialize + TupleList> BinarySerialize for (Head, Tail) {
    fn write_to(&self, writer: &mut BinaryWriter) {
        self.0.write_to(writer);
        self.1.write_to(writer);
    }
}