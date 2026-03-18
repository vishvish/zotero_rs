//! `POST https://www.zotero.org/oauth/access`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::oauth::signing::{
    authorization_header, build_signature_base_string, hmac_sha1_signature, oauth_nonce,
    oauth_timestamp,
};
use crate::requests::oauth_access_token_request::OAuthAccessTokenRequest;
use crate::types::oauth::OAuthAccessToken;

impl ZoteroClient {
    /// Exchanges a request token for an access token.
    pub async fn oauth_access_token(
        &self,
        request: &OAuthAccessTokenRequest,
    ) -> Result<OAuthAccessToken, ZoteroClientError> {
        let endpoint = "https://www.zotero.org/oauth/access";
        let nonce = oauth_nonce();
        let timestamp = oauth_timestamp();

        let mut oauth_params = vec![
            (
                "oauth_consumer_key".to_owned(),
                request.consumer_key.clone(),
            ),
            ("oauth_nonce".to_owned(), nonce),
            ("oauth_signature_method".to_owned(), "HMAC-SHA1".to_owned()),
            ("oauth_timestamp".to_owned(), timestamp),
            ("oauth_token".to_owned(), request.request_token.clone()),
            ("oauth_verifier".to_owned(), request.verifier.clone()),
            ("oauth_version".to_owned(), "1.0".to_owned()),
        ];

        let signature_base = build_signature_base_string("POST", endpoint, &oauth_params);
        let signature = hmac_sha1_signature(
            &signature_base,
            &request.consumer_secret,
            Some(&request.request_token_secret),
        );
        oauth_params.push(("oauth_signature".to_owned(), signature));

        let authorization = authorization_header(&oauth_params);

        let response = self
            .http
            .post(endpoint)
            .header("Authorization", authorization)
            .send()
            .await?;

        let body = response.text().await?;
        parse_access_token_response(&body)
    }
}

fn parse_access_token_response(body: &str) -> Result<OAuthAccessToken, ZoteroClientError> {
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

    Ok(OAuthAccessToken {
        token: token.clone(),
        token_secret: token_secret.clone(),
        user_id: map.get("userID").cloned(),
        username: map.get("username").cloned(),
    })
}

#[cfg(test)]
mod tests {
    use crate::api::oauth::access_token::parse_access_token_response;

    #[test]
    fn parses_access_token_response() {
        let parsed = parse_access_token_response(
            "oauth_token=t&oauth_token_secret=s&userID=99&username=test-user",
        )
        .expect("must parse");

        assert_eq!(parsed.token, "t");
        assert_eq!(parsed.token_secret, "s");
        assert_eq!(parsed.user_id.as_deref(), Some("99"));
        assert_eq!(parsed.username.as_deref(), Some("test-user"));
    }

    #[test]
    fn fails_on_missing_access_token() {
        let err = parse_access_token_response("oauth_token_secret=s").expect_err("must fail");
        assert!(format!("{err}").contains("oauth_token"));
    }
}
