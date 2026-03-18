//! URL path safety helpers.

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// Encodes a user-provided path segment for safe URL construction.
pub(crate) fn encode_path_segment(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use crate::client::path::encode_path_segment;

    #[test]
    fn encodes_reserved_path_characters() {
        assert_eq!(encode_path_segment("A/B C"), "A%2FB%20C");
    }
}
