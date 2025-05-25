pub mod byte_sized;
pub mod deserialize;
pub mod osu_types;
pub mod serialize;
pub mod uleb128;

pub use deserialize::{BinaryDeserialize, BinaryReader};
pub use serialize::{BinarySerialize, BinaryWriter};

pub mod macros {
    pub use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized, Message};
}
