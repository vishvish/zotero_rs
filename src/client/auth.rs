//! Authentication strategy and request header injection.

use reqwest::RequestBuilder;
use secrecy::{ExposeSecret, SecretString};

/// Authentication strategy.
#[derive(Clone, Debug)]
pub enum Auth {
    /// Uses `Zotero-API-Key: <key>`.
    ApiKey(SecretString),
    /// Uses `Authorization: Bearer <token>`.
    BearerToken(SecretString),
}

pub(crate) fn apply_auth(builder: RequestBuilder, auth: &Option<Auth>) -> RequestBuilder {
    match auth {
        Some(Auth::ApiKey(key)) => builder.header("Zotero-API-Key", key.expose_secret()),
        Some(Auth::BearerToken(token)) => builder.bearer_auth(token.expose_secret()),
        None => builder,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injects_api_key_header() {
        let client = reqwest::Client::new();
        let request = apply_auth(
            client.get("https://api.zotero.org/users/1/items"),
            &Some(Auth::ApiKey(SecretString::from("test-key".to_owned()))),
        )
        .build()
        .expect("request should build");

        let header = request
            .headers()
            .get("Zotero-API-Key")
            .and_then(|value| value.to_str().ok());
        assert_eq!(header, Some("test-key"));
    }

    #[test]
    fn injects_bearer_header() {
        let client = reqwest::Client::new();
        let request = apply_auth(
            client.get("https://api.zotero.org/users/1/items"),
            &Some(Auth::BearerToken(SecretString::from(
                "test-token".to_owned(),
            ))),
        )
        .build()
        .expect("request should build");

        let header = request
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok());
        assert_eq!(header, Some("Bearer test-token"));
    }
}
