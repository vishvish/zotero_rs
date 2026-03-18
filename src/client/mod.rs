//! Core client internals and shared request execution helpers.

mod auth;
mod config;
mod error;
mod fetch_next_page;
mod headers;
mod new_client;
mod path;
mod request_execution;
mod retry_policy;
mod sync_conflict_retry;

#[cfg(test)]
mod request_execution_tests;

use reqwest::Client as HttpClient;
use url::Url;

pub use auth::Auth;
pub use config::{ClientOptions, RetryPolicy};
pub use error::ZoteroClientError;
pub(crate) use path::encode_path_segment;

/// Main HTTP client for Zotero Web API v3.
pub struct ZoteroClient {
    pub(crate) http: HttpClient,
    pub(crate) options: ClientOptions,
    pub(crate) base_url: Url,
}
