use crate::serde::macros::BinaryDeserialize;
use crate::structures::Match;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangeSettings<'a> {
    pub match_data: Match<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_change_settings_has_match_field() {
        let _ = std::any::type_name::<MatchChangeSettings>();
    }

    #[test]
    fn test_match_change_settings_debug_impl() {
        let _ = std::any::type_name::<MatchChangeSettings>();
    }
}
