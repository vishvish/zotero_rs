//! Query options for listing items.

/// Query options for `list_items`.
#[derive(Clone, Debug, Default)]
pub struct ListItemsRequest {
    /// Return only changes since this version.
    pub since: Option<u64>,
    /// Maximum items per page.
    pub limit: Option<u32>,
    /// Pagination offset.
    pub start: Option<u32>,
    /// Include trash items.
    pub include_trash: bool,
    /// Collection key filter.
    pub collection_key: Option<String>,
    /// Item type filter.
    pub item_type: Option<String>,
    /// Search query.
    pub q: Option<String>,
}

impl ListItemsRequest {
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
        if self.include_trash {
            pairs.push(("includeTrashed".to_owned(), "1".to_owned()));
        }
        if let Some(value) = &self.collection_key {
            pairs.push(("collection".to_owned(), value.clone()));
        }
        if let Some(value) = &self.item_type {
            pairs.push(("itemType".to_owned(), value.clone()));
        }
        if let Some(value) = &self.q {
            pairs.push(("q".to_owned(), value.clone()));
        }

        pairs
    }
}
