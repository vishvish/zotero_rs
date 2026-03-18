//! OAuth 1.0a signing helpers.

use base64::Engine as _;
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::distr::{Alphanumeric, SampleString};
use sha1::Sha1;

const OAUTH_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

/// Percent-encodes a value according to OAuth 1.0a rules.
pub fn percent_encode_oauth(value: &str) -> String {
    utf8_percent_encode(value, OAUTH_ENCODE_SET).to_string()
}

/// Generates a random OAuth nonce.
pub fn oauth_nonce() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 32)
}

/// Creates a UNIX timestamp string for OAuth headers.
pub fn oauth_timestamp() -> String {
    chrono_like_unix_seconds().to_string()
}

/// Builds a sorted OAuth signature base string.
pub fn build_signature_base_string(method: &str, url: &str, params: &[(String, String)]) -> String {
    let mut sorted = params.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let parameter_string = sorted
        .into_iter()
        .map(|(k, v)| format!("{}={}", percent_encode_oauth(&k), percent_encode_oauth(&v)))
        .collect::<Vec<_>>()
        .join("&");

    format!(
        "{}&{}&{}",
        method.to_uppercase(),
        percent_encode_oauth(url),
        percent_encode_oauth(&parameter_string)
    )
}

/// Creates an OAuth HMAC-SHA1 signature.
pub fn hmac_sha1_signature(
    base_string: &str,
    consumer_secret: &str,
    token_secret: Option<&str>,
) -> String {
    let key = format!(
        "{}&{}",
        percent_encode_oauth(consumer_secret),
        percent_encode_oauth(token_secret.unwrap_or(""))
    );

    let mut mac = Hmac::<Sha1>::new_from_slice(key.as_bytes()).expect("valid hmac key length");
    mac.update(base_string.as_bytes());
    let sig = mac.finalize().into_bytes();

    base64::engine::general_purpose::STANDARD.encode(sig)
}

/// Builds an OAuth Authorization header value.
pub fn authorization_header(oauth_pairs: &[(String, String)]) -> String {
    let mut pairs = oauth_pairs.to_vec();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let joined = pairs
        .iter()
        .map(|(k, v)| {
            format!(
                "{}=\"{}\"",
                percent_encode_oauth(k),
                percent_encode_oauth(v)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!("OAuth {joined}")
}

fn chrono_like_unix_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::oauth::signing::{build_signature_base_string, percent_encode_oauth};

    #[test]
    fn percent_encodes_oauth_values() {
        assert_eq!(percent_encode_oauth("a b+c"), "a%20b%2Bc");
    }

    #[test]
    fn creates_deterministic_base_string() {
        let params = vec![
            ("oauth_consumer_key".to_owned(), "key".to_owned()),
            ("oauth_nonce".to_owned(), "nonce".to_owned()),
            ("oauth_signature_method".to_owned(), "HMAC-SHA1".to_owned()),
            ("oauth_timestamp".to_owned(), "1700000000".to_owned()),
            ("oauth_version".to_owned(), "1.0".to_owned()),
        ];

        let base =
            build_signature_base_string("POST", "https://www.zotero.org/oauth/request", &params);

        assert!(base.starts_with("POST&https%3A%2F%2Fwww.zotero.org%2Foauth%2Frequest&"));
    }
}
