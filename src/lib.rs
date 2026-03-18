#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! `zotero-rs` is a secure, decomposed Rust client for Zotero Web API v3.

/// API calls grouped by Zotero resource.
pub mod api;
/// Core HTTP client and error types.
pub mod client;
/// OAuth utilities.
pub mod oauth;
/// Request option types.
pub mod requests;
/// Response wrappers and metadata.
pub mod responses;
/// Shared domain types.
pub mod types;

pub use client::{Auth, ClientOptions, RetryPolicy, ZoteroClient, ZoteroClientError};
pub use requests::write_options::WriteOptions;
pub use types::library_scope::LibraryScope;
