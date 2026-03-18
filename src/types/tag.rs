//! Tag response objects.

use serde::{Deserialize, Serialize};

/// Tag entry returned by `tags` endpoints.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
    /// Tag text.
    pub tag: String,
    /// Tag type identifier.
    #[serde(default)]
    pub r#type: i32,
}
