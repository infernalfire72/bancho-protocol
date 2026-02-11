use crate::serde::macros::BinaryDeserialize;
use crate::structures::Match;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct CreateMatch<'a> {
    pub match_data: Match<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_match_struct_exists() {
        // Simply verify the struct exists and is usable
        let _ = std::any::type_name::<CreateMatch>();
    }
}
