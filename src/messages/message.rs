use super::MessageType;
use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};

pub const HEADER_SIZE: usize = 7;

#[derive(Debug)]
pub struct MessageHeader {
    pub message_type: MessageType,
    _compress: bool,
    pub args_len: u32,
}

impl<'a> BinaryDeserialize<'a> for MessageHeader {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> {
        let message_type = MessageType::read_from(reader)?;
        let _compress = bool::read_from(reader)?;
        let args_len = u32::read_from(reader)?;
        Ok(Self {
            message_type,
            _compress,
            args_len,
        })
    }
}

pub trait MessageArgs: Sized + BinarySerialize {
    const MESSAGE_TYPE: MessageType;
    fn as_message(&self) -> Message<Self> {
        Message(self)
    }
}

/// A general interface for bancho packets.
/// Never construct this manually.
#[derive(Debug)]
pub struct Message<'a, Args: MessageArgs>(pub &'a Args);

impl<Args: MessageArgs> Message<'_, Args> {
    pub fn serialize(args: Args) -> Vec<u8> {
        let msg = args.as_message();
        msg.serialize()
    }
}

impl<Args: MessageArgs> ByteSized for Message<'_, Args> {
    fn byte_size(&self) -> usize {
        HEADER_SIZE + self.0.byte_size()
    }
}

impl<Args: MessageArgs> BinarySerialize for Message<'_, Args> {
    fn write_to(&self, mut writer: &mut BinaryWriter) {
        let msg_id = Args::MESSAGE_TYPE as u16;
        let args_len = self.0.byte_size() as u32;
        // header
        msg_id.write_to(&mut writer);
        writer.write_byte(0);
        args_len.write_to(&mut writer);

        self.0.write_to(&mut writer);
    }
}

impl<Args: MessageArgs> ByteSized for (Args,) {
    fn byte_size(&self) -> usize {
        self.0.as_message().byte_size()
    }
}

impl<Args: MessageArgs> BinarySerialize for (Args,) {
    fn write_to(&self, writer: &mut BinaryWriter) {
        self.0.as_message().write_to(writer);
    }
}

impl<Args: MessageArgs, B: ByteSized> ByteSized for (Args, B) {
    fn byte_size(&self) -> usize {
        self.0.as_message().byte_size() + self.1.byte_size()
    }
}

impl<Args: MessageArgs, B: BinarySerialize> BinarySerialize for (Args, B) {
    fn write_to(&self, writer: &mut BinaryWriter) {
        self.0.as_message().write_to(writer);
        self.1.write_to(writer);
    }
}

#[macro_export]
macro_rules! expand_into_tuple {
    () => { () };
    ($e:expr) => { ($e,) };
    ($e:expr, $($rest:expr),+) => {
        ($e, expand_into_tuple!($($rest),+))
    };
}

#[macro_export]
macro_rules! concat_messages {
    ($($e:expr),+ $(,)?) => {
        {
            use $crate::expand_into_tuple;
            use $crate::serde::BinarySerialize;
            expand_into_tuple!($($e),+).serialize()
        }
    };
    [$e:expr; $n:expr] => {
        {
            let msg = $e.as_message();
            let data_len = msg.byte_size() * $n;
            let mut writer = BinaryWriter::with_length(data_len);
            for _ in 0..$n {
                msg.write_to(&mut writer);
            }
            writer.data()
        }
    }
}
