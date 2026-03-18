//! `GET https://www.zotero.org/oauth/authorize`

use crate::client::ZoteroClient;

impl ZoteroClient {
    /// Builds user-authorization URL for a request token.
    pub fn oauth_authorize_url(&self, request_token: &str) -> String {
        let mut url = url::Url::parse("https://www.zotero.org/oauth/authorize")
            .expect("valid OAuth authorize URL");
        url.query_pairs_mut()
            .append_pair("oauth_token", request_token);
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{ClientOptions, ZoteroClient};

    #[test]
    fn builds_authorize_url_with_token() {
        let client = ZoteroClient::new(ClientOptions::default()).expect("client");
        let url = client.oauth_authorize_url("abc123");
        assert_eq!(
            url,
            "https://www.zotero.org/oauth/authorize?oauth_token=abc123"
        );
    }
}
