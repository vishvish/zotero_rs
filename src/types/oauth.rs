//! OAuth token types.

/// Request token pair returned by Zotero OAuth endpoints.
#[derive(Clone, Debug)]
pub struct OAuthToken {
    /// OAuth token value.
    pub token: String,
    /// OAuth token secret.
    pub token_secret: String,
}

/// Access token response from Zotero OAuth access endpoint.
#[derive(Clone, Debug)]
pub struct OAuthAccessToken {
    /// Issued API key token.
    pub token: String,
    /// Issued API key secret.
    pub token_secret: String,
    /// Zotero user identifier.
    pub user_id: Option<String>,
    /// Zotero username.
    pub username: Option<String>,
}
