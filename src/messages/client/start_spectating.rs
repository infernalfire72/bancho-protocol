use crate::serde::macros::BinaryDeserialize;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct StartSpectating {
    pub target_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::BinaryDeserialize;

    #[test]
    fn test_start_spectating_zero_id() {
        let data = [0, 0, 0, 0];
        let msg = StartSpectating::deserialize(&data).unwrap();
        assert_eq!(msg.target_id, 0);
    }

    #[test]
    fn test_start_spectating_valid_id() {
        let data = [123, 4, 0, 0];
        let msg = StartSpectating::deserialize(&data).unwrap();
        assert_eq!(msg.target_id, 1147);
    }

    #[test]
    fn test_start_spectating_debug_format() {
        let msg = StartSpectating { target_id: 99 };
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("99"));
    }
}
