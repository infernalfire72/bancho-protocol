use crate::serde::byte_sized::ByteSized;
use super::MessageType;
use crate::serde::serialize::{BinarySerialize, BinaryWriter};
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};


pub const HEADER_SIZE: usize = 7;

#[derive(Debug)]
pub struct MessageHeader {
    pub message_type: MessageType,
    _compress: bool,
    pub args_len: u32,
}

impl<'a> BinaryDeserialize<'a> for MessageHeader {
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self> where Self: Sized {
        unsafe {
            let ptr = reader.next_range(HEADER_SIZE)?.as_ptr();
            let message_type = std::ptr::read_unaligned(ptr as *const MessageType);
            let _compress = std::ptr::read_unaligned(ptr.offset(2) as *const bool);
            let args_len = std::ptr::read_unaligned(ptr.offset(3) as *const u32);

            Ok(Self { message_type, _compress, args_len })
        }
    }
}

/// A general interface for bancho packets.
/// Never construct this manually.
#[derive(Debug)]
pub struct Message<const M: MessageType, T: BinarySerialize>(pub T);

impl<const M: MessageType, T: BinarySerialize> Message<M, T> {
    pub fn serialize(args: impl Into<Message<M, T>>) -> Vec<u8> {
        let msg = args.into();
        let mut writer = BinaryWriter::with_length(msg.byte_size());
        msg.write_to(&mut writer);
        writer.data()
    }
}

impl<const M: MessageType, T: BinarySerialize> ByteSized for Message<M, T> {
    fn byte_size(&self) -> usize {
        HEADER_SIZE + self.0.byte_size()
    }
}

impl<const M: MessageType, T: BinarySerialize> BinarySerialize for Message<M, T> {
    fn write_to(&self, mut writer: &mut BinaryWriter) {
        // header
        u16::write_to(&(M as _), &mut writer);
        writer.write_byte(0);
        u32::write_to(&(self.0.byte_size() as _), &mut writer);

        self.0.write_to(&mut writer);
    }
}

#[macro_export]
macro_rules! concat_messages {
    ($($e:expr),+) => {
        {
            use tuple_list::{tuple_list};
            tuple_list!(
                $({
                    let msg: Message<_, _> = $e.into();
                    msg
                }),+
            ).serialize()
        }
    };
    [$e:expr; $n:expr] => {
        {
            let msg: Message<_, _> = $e.into();
            let data_len = msg.byte_size() * $n;
            let mut writer = BinaryWriter::with_length(data_len);
            for _ in 0..$n {
                msg.write_to(&mut writer);
            }
            writer.data()
        }
    }
}