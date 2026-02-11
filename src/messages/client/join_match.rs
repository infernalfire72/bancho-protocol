use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct JoinMatch<'a> {
    pub match_id: i32,
    pub password: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_join_match_zero_id_empty_password() {
        let data = [0, 0, 0, 0, 0];
        let msg = JoinMatch::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 0);
        assert_eq!(msg.password, "");
    }

    #[test]
    fn test_join_match_with_id_and_password() {
        use crate::serde::{BinarySerialize, BinaryWriter};
        use crate::serde::byte_sized::ByteSized;
        // match_id = 42, password = "test"
        let match_id = 42i32;
        let password = "test";

        let size = match_id.byte_size() + password.byte_size();
        let mut writer = BinaryWriter::with_length(size);
        match_id.write_to(&mut writer);
        password.write_to(&mut writer);
        let data = writer.data();

        let msg = JoinMatch::deserialize(&data).unwrap();
        assert_eq!(msg.match_id, 42);
        assert_eq!(msg.password, "test");
    }

    #[test]
    fn test_join_match_debug_format() {
        let data = [0, 0, 0, 0, 0];
        let msg = JoinMatch::deserialize(&data).unwrap();
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("JoinMatch"));
    }
}
