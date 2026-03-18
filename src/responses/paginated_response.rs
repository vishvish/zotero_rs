//! Paginated endpoint response wrapper.

use crate::responses::response_metadata::ResponseMetadata;

/// Generic paginated response payload.
#[derive(Clone, Debug)]
pub struct PaginatedResponse<T> {
    /// Page items.
    pub data: Vec<T>,
    /// Parsed response metadata.
    pub metadata: ResponseMetadata,
}

impl<T> PaginatedResponse<T> {
    /// Returns true when the response contains a `rel="next"` link.
    pub fn has_next_page(&self) -> bool {
        self.metadata.links.next.is_some()
    }

    /// Returns the absolute URL for the next page when present.
    pub fn next_page_url(&self) -> Option<&str> {
        self.metadata.links.next.as_deref()
    }
}
