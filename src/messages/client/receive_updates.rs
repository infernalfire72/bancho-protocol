use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct ReceiveUpdates {
    pub filter: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_receive_updates_no_filter() {
        let data = [0, 0, 0, 0];
        let msg = ReceiveUpdates::deserialize(&data).unwrap();
        assert_eq!(msg.filter, 0);
    }

    #[test]
    fn test_receive_updates_with_filter() {
        let data = [255, 0, 0, 0];
        let msg = ReceiveUpdates::deserialize(&data).unwrap();
        assert_eq!(msg.filter, 255);
    }

    #[test]
    fn test_receive_updates_all_bits() {
        let data = [255, 255, 255, 255];
        let msg = ReceiveUpdates::deserialize(&data).unwrap();
        assert_eq!(msg.filter, u32::MAX);
    }
}
