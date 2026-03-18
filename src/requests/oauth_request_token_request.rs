//! Request parameters for OAuth request-token call.

/// Inputs used to request a temporary OAuth token.
#[derive(Clone, Debug)]
pub struct OAuthRequestTokenRequest {
    /// OAuth consumer key.
    pub consumer_key: String,
    /// OAuth consumer secret.
    pub consumer_secret: String,
    /// OAuth callback URL.
    pub callback: String,
    /// Application name displayed by Zotero.
    pub name: String,
    /// Library scope, e.g. `all`.
    pub library_access: String,
    /// Whether all groups are requested.
    pub all_groups: bool,
    /// Whether write access is requested.
    pub write_access: bool,
}
