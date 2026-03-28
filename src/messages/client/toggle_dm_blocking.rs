use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ToggleBlockNonFriendDms {
    pub val: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_toggle_dm_blocking_disabled() {
        let data = [0, 0, 0, 0];
        let msg = ToggleBlockNonFriendDms::deserialize(&data).unwrap();
        assert_eq!(msg.val, 0);
    }

    #[test]
    fn test_toggle_dm_blocking_enabled() {
        let data = [1, 0, 0, 0];
        let msg = ToggleBlockNonFriendDms::deserialize(&data).unwrap();
        assert_eq!(msg.val, 1);
    }
}
