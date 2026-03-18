//! Query options for listing tags.

/// Query options for `list_tags`.
#[derive(Clone, Debug, Default)]
pub struct ListTagsRequest {
    /// Return only changes since this version.
    pub since: Option<u64>,
    /// Maximum tags per page.
    pub limit: Option<u32>,
    /// Pagination offset.
    pub start: Option<u32>,
}

impl ListTagsRequest {
    /// Returns URL query pairs.
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();

        if let Some(value) = self.since {
            pairs.push(("since".to_owned(), value.to_string()));
        }
        if let Some(value) = self.limit {
            pairs.push(("limit".to_owned(), value.to_string()));
        }
        if let Some(value) = self.start {
            pairs.push(("start".to_owned(), value.to_string()));
        }

        pairs
    }
}
