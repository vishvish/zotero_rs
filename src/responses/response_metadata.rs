//! Metadata extracted from Zotero headers.

use crate::responses::pagination_links::PaginationLinks;

/// Metadata extracted from HTTP response headers.
#[derive(Clone, Debug, Default)]
pub struct ResponseMetadata {
    /// `Last-Modified-Version` header value.
    pub last_modified_version: Option<u64>,
    /// `Zotero-Library-Version` header value.
    pub library_version: Option<u64>,
    /// `Backoff` header in seconds.
    pub backoff_seconds: Option<u64>,
    /// `Retry-After` header in seconds.
    pub retry_after_seconds: Option<u64>,
    /// Parsed pagination links.
    pub links: PaginationLinks,
}
