//! `POST https://www.zotero.org/oauth/request`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::oauth::signing::{
    authorization_header, build_signature_base_string, hmac_sha1_signature, oauth_nonce,
    oauth_timestamp,
};
use crate::requests::oauth_request_token_request::OAuthRequestTokenRequest;
use crate::types::oauth::OAuthToken;

impl ZoteroClient {
    /// Requests a temporary OAuth token.
    pub async fn oauth_request_token(
        &self,
        request: &OAuthRequestTokenRequest,
    ) -> Result<OAuthToken, ZoteroClientError> {
        let endpoint = "https://www.zotero.org/oauth/request";
        let nonce = oauth_nonce();
        let timestamp = oauth_timestamp();

        let oauth_params = vec![
            ("oauth_callback".to_owned(), request.callback.clone()),
            (
                "oauth_consumer_key".to_owned(),
                request.consumer_key.clone(),
            ),
            ("oauth_nonce".to_owned(), nonce.clone()),
            ("oauth_signature_method".to_owned(), "HMAC-SHA1".to_owned()),
            ("oauth_timestamp".to_owned(), timestamp.clone()),
            ("oauth_version".to_owned(), "1.0".to_owned()),
        ];

        let body_params = vec![
            ("name".to_owned(), request.name.clone()),
            ("library_access".to_owned(), request.library_access.clone()),
            (
                "all_groups".to_owned(),
                if request.all_groups {
                    "write".to_owned()
                } else {
                    "none".to_owned()
                },
            ),
            (
                "write_access".to_owned(),
                if request.write_access {
                    "1".to_owned()
                } else {
                    "0".to_owned()
                },
            ),
        ];

        let mut signature_params = oauth_params.clone();
        signature_params.extend(body_params.clone());

        let signature_base = build_signature_base_string("POST", endpoint, &signature_params);
        let signature = hmac_sha1_signature(&signature_base, &request.consumer_secret, None);

        let mut header_pairs = oauth_params;
        header_pairs.push(("oauth_signature".to_owned(), signature));

        let authorization = authorization_header(&header_pairs);

        let response = self
            .http
            .post(endpoint)
            .header("Authorization", authorization)
            .form(&body_params)
            .send()
            .await?;

        let body = response.text().await?;
        parse_request_token_response(&body)
    }
}

fn parse_request_token_response(body: &str) -> Result<OAuthToken, ZoteroClientError> {
    let map = url::form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect::<std::collections::HashMap<String, String>>();

    let Some(token) = map.get("oauth_token") else {
        return Err(ZoteroClientError::InvalidOAuthResponse(
            "missing oauth_token".into(),
        ));
    };
    let Some(token_secret) = map.get("oauth_token_secret") else {
        return Err(ZoteroClientError::InvalidOAuthResponse(
            "missing oauth_token_secret".into(),
        ));
    };

    Ok(OAuthToken {
        token: token.clone(),
        token_secret: token_secret.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::api::oauth::request_token::parse_request_token_response;
    use crate::oauth::signing::percent_encode_oauth;

    #[test]
    fn oauth_encode_keeps_expected_characters_encoded() {
        assert_eq!(
            percent_encode_oauth("https://example.com/callback?a=1&b=2"),
            "https%3A%2F%2Fexample.com%2Fcallback%3Fa%3D1%26b%3D2"
        );
    }

    #[test]
    fn parses_request_token_response() {
        let parsed =
            parse_request_token_response("oauth_token=a&oauth_token_secret=b").expect("must parse");
        assert_eq!(parsed.token, "a");
        assert_eq!(parsed.token_secret, "b");
    }

    #[test]
    fn fails_on_missing_request_token_secret() {
        let err = parse_request_token_response("oauth_token=a").expect_err("must fail");
        assert!(format!("{err}").contains("oauth_token_secret"));
    }
}
