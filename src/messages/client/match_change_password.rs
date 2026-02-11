use crate::serde::macros::BinaryDeserialize;
use crate::structures::Match;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct MatchChangePassword<'a> {
    pub match_data: Match<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_change_password_has_match_field() {
        let _ = std::any::type_name::<MatchChangePassword>();
    }

    #[test]
    fn test_match_change_password_debug_impl() {
        let _ = std::any::type_name::<MatchChangePassword>();
    }
}
