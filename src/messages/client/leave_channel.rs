use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct LeaveChannel<'a> {
    pub name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_leave_channel_empty() {
        let data = [0];
        let msg = LeaveChannel::deserialize(&data).unwrap();
        assert_eq!(msg.name, "");
    }

    #[test]
    fn test_leave_channel_valid() {
        use crate::serde::{BinarySerialize, BinaryWriter};
        use crate::serde::byte_sized::ByteSized;
        let name_str = "#main";
        // Serialize to get proper format
        let mut writer = BinaryWriter::with_length(name_str.byte_size());
        name_str.write_to(&mut writer);
        let data = writer.data();
        let msg = LeaveChannel::deserialize(&data).unwrap();
        assert_eq!(msg.name, "#main");
    }

    #[test]
    fn test_leave_channel_debug_format() {
        use crate::serde::{BinarySerialize, BinaryWriter};
        use crate::serde::byte_sized::ByteSized;
        let name_str = "foo";
        let mut writer = BinaryWriter::with_length(name_str.byte_size());
        name_str.write_to(&mut writer);
        let data = writer.data();
        let msg = LeaveChannel::deserialize(&data).unwrap();
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("foo"));
    }
}
