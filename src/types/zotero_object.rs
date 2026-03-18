//! Generic Zotero object envelope.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::item_metadata::ItemMetadata;

/// Generic object shape returned by many Zotero endpoints.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoteroObject {
    /// Item/collection/search key.
    pub key: String,
    /// Server object version.
    pub version: u64,
    /// Resource payload.
    #[serde(default)]
    pub data: Value,
}

impl ZoteroObject {
    /// Converts the generic item payload into a structured metadata object.
    pub fn to_item_metadata(&self) -> ItemMetadata {
        ItemMetadata::from(self)
    }
}
