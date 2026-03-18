//! Parsed HTTP Link header relations.

/// Pagination links parsed from `Link` header.
#[derive(Clone, Debug, Default)]
pub struct PaginationLinks {
    /// `rel=first`.
    pub first: Option<String>,
    /// `rel=prev`.
    pub prev: Option<String>,
    /// `rel=next`.
    pub next: Option<String>,
    /// `rel=last`.
    pub last: Option<String>,
    /// `rel=alternate`.
    pub alternate: Option<String>,
}
