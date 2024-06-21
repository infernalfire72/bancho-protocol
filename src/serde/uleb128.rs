use std::fmt::{Debug, Display, Formatter};
use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};

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
            if v != 0 {
                b |= 0x80;
            }
            writer.write_byte(b);
            if v == 0 {
                break;
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
