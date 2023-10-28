use crate::serde::uleb128::v32;

use super::ByteStream;

pub trait BinaryDeserialize {
    fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> where Self: Sized;
}

macro_rules! impl_deserialize {
    ($t:ty) => {
        impl BinaryDeserialize for $t {
            fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> {
                Ok(<$t>::from_le_bytes(*reader.next_range_cty::<{std::mem::size_of::<$t>()}>()?))
            }
        }
    };
    ($t:ty, $($tt:ty), +) => {
        impl_deserialize!($t);
        impl_deserialize!($($tt), +);
    }
}

impl BinaryDeserialize for u8 {
    fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> {
        reader.next()
    }
}

impl BinaryDeserialize for bool {
    fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> {
        Ok(reader.next()? == 1)
    }
}

impl_deserialize!(u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl BinaryDeserialize for String {
    fn read_from(mut reader: &mut ByteStream) -> std::io::Result<Self> {
        let osu_type = reader.next()?;
        if osu_type != 0x0b {
            return Ok(Self::default())
        }

        let len = v32::read_from(&mut reader)?;
        let bytes = reader.next_range(len.0 as usize)?;
        Ok(String::from_utf8_lossy(bytes).to_string())
    }
}

impl<const N: usize, T: BinaryDeserialize + Copy> BinaryDeserialize for [T; N] {
    fn read_from(reader: &mut ByteStream) -> std::io::Result<Self> where Self: Sized {
        let init = T::read_from(reader)?;
        let mut r = [init; N];
        for i in 1..N {
            r[i] = T::read_from(reader)?;
        }
        Ok(r)
    }
}