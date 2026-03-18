//! Collection response objects.

use serde::{Deserialize, Serialize};

use crate::types::zotero_object::ZoteroObject;

/// Collection envelope.
pub type Collection = ZoteroObject;

/// Body for creating/updating collections.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionInput {
    /// Resource payload sent to Zotero.
    #[serde(flatten)]
    pub data: serde_json::Value,
}
