//! Request parameters for OAuth access-token call.

/// Inputs used to exchange a request token for an access token.
#[derive(Clone, Debug)]
pub struct OAuthAccessTokenRequest {
    /// OAuth consumer key.
    pub consumer_key: String,
    /// OAuth consumer secret.
    pub consumer_secret: String,
    /// Request token.
    pub request_token: String,
    /// Request token secret.
    pub request_token_secret: String,
    /// OAuth verifier from callback.
    pub verifier: String,
}
