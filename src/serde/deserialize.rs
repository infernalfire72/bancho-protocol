use crate::serde::uleb128::v32;

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
        Ok(unsafe { std::str::from_utf8_unchecked(bytes) })
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
