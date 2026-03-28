use crate::messages::MessageType;
use crate::serde::macros::{BinarySerialize, ByteSized, Message};

#[derive(Debug, BinarySerialize, ByteSized, Message)]
#[crate_root(crate)]
#[message(MessageType::UserLogout)]
pub struct UserLogout {
    user_id: i64, // this is usually user_id: i32, state: u8
}

impl UserLogout {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id: user_id as _,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::message::MessageArgs;
    use crate::serde::byte_sized::ByteSized;
    use crate::serde::BinarySerialize;

    #[test]
    fn test_user_logout_new() {
        let msg = UserLogout::new(42);
        assert_eq!(msg.user_id, 42);
    }

    #[test]
    fn test_user_logout_new_stores_as_i64() {
        let msg = UserLogout::new(999);
        assert_eq!(msg.user_id, 999i64);
    }

    #[test]
    fn test_user_logout_byte_size() {
        let msg = UserLogout::new(1);
        assert_eq!(msg.byte_size(), 8); // i64
    }

    #[test]
    fn test_user_logout_serialize() {
        let msg = UserLogout::new(42);
        let bytes = msg.serialize();
        assert_eq!(bytes, 42i64.to_le_bytes());
    }

    #[test]
    fn test_user_logout_serialize_zero() {
        let msg = UserLogout::new(0);
        let bytes = msg.serialize();
        assert_eq!(bytes, vec![0x00; 8]);
    }

    #[test]
    fn test_user_logout_message_type() {
        assert_eq!(UserLogout::MESSAGE_TYPE, MessageType::UserLogout);
    }
}
