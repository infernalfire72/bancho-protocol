use std::io::Error;
use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[repr(u8)]
#[derive(Debug, Copy, Clone, BinaryDeserialize, BinarySerialize, ByteSized)]
#[crate_root(crate)]
pub enum Mode {
    Standard,
    Taiko,
    Catch,
    Mania,
}

impl TryFrom<u8> for Mode {
    type Error = Error;

    fn try_from(mut value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            value = 0;
        }

        // SAFETY: `Mode` has 4 variants
        Ok(unsafe { std::mem::transmute(value) })
    }
}