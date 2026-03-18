//! Client construction options.

use std::time::Duration;

/// Retry behavior for transient API failures.
#[derive(Clone, Copy, Debug)]
pub struct RetryPolicy {
    /// Total attempts including the first request.
    pub max_attempts: u32,
    /// Base exponential backoff delay.
    pub base_delay: Duration,
    /// Maximum backoff delay.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
        }
    }
}

/// Options used to create a [`crate::client::ZoteroClient`].
#[derive(Clone, Debug)]
pub struct ClientOptions {
    /// API endpoint root.
    pub base_url: String,
    /// Optional authentication.
    pub auth: Option<crate::client::Auth>,
    /// Total request timeout.
    pub timeout: Duration,
    /// Connection establishment timeout.
    pub connect_timeout: Duration,
    /// `User-Agent` value.
    pub user_agent: String,
    /// Retry policy.
    pub retry_policy: RetryPolicy,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            base_url: "https://api.zotero.org/".to_owned(),
            auth: None,
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            user_agent: format!("zotero-rs/{}", env!("CARGO_PKG_VERSION")),
            retry_policy: RetryPolicy::default(),
        }
    }
}
