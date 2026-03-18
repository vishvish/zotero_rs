//! Deleted object keys from sync endpoint.

use serde::{Deserialize, Serialize};

/// Deleted keys grouped by type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletedResponse {
    /// Collection keys.
    #[serde(default)]
    pub collections: Vec<String>,
    /// Search keys.
    #[serde(default)]
    pub searches: Vec<String>,
    /// Item keys.
    #[serde(default)]
    pub items: Vec<String>,
    /// Tag keys.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Settings keys.
    #[serde(default)]
    pub settings: Vec<String>,
}
