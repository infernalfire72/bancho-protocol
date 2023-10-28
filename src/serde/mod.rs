pub mod byte_sized;
pub mod deserialize;
pub mod serialize;
pub mod uleb128;
pub mod osu_types;

pub struct ByteStream<'a> {
    mem: &'a [u8],
    position: usize,
}

impl<'a> ByteStream<'a> {
    pub fn new(mem: &'a [u8]) -> Self {
        Self {
            mem,
            position: 0,
        }
    }

    pub fn can_read(&self) -> bool {
        self.position < self.mem.len()
    }

    pub fn can_read_n(&self, count: usize) -> bool {
        self.position + count - 1 < self.mem.len()
    }

    pub fn next(&mut self) -> std::io::Result<u8> {
        if !self.can_read() {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "unexpected end of stream"));
        }

        let b = self.mem[self.position];
        self.position += 1;
        Ok(b)
    }

    pub fn next_range(&mut self, count: usize) -> std::io::Result<&[u8]> {
        if !self.can_read_n(count) {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "unexpected end of stream"));
        }

        let e = self.position + count;
        let r = &self.mem[self.position..e];
        self.position = e;
        Ok(r)
    }

    pub fn next_range_cty<const COUNT: usize>(&mut self) -> std::io::Result<&[u8; COUNT]> {
        Ok(self.next_range(COUNT)?.first_chunk::<COUNT>().unwrap())
    }
}


pub mod macros {
    pub use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized, Message};
}