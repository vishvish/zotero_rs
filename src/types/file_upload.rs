//! File-upload response types.

use serde::{Deserialize, Serialize};

/// Request body for file upload authorization.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileUploadAuthorizationRequest {
    /// Item filename.
    pub filename: String,
    /// File size in bytes.
    pub filesize: u64,
    /// File MD5 hash.
    pub md5: String,
    /// File modification time (ms since epoch).
    pub mtime: u64,
    /// Optional MIME type.
    pub content_type: Option<String>,
    /// Optional charset.
    pub charset: Option<String>,
}

/// Response from file upload authorization call.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileUploadAuthorizationResponse {
    /// Whether file already exists remotely.
    #[serde(default)]
    pub exists: i32,
    /// Upload key for final registration.
    #[serde(default)]
    pub upload_key: Option<String>,
    /// Target upload URL for binary data.
    #[serde(default)]
    pub url: Option<String>,
    /// Prefix bytes wrapper.
    #[serde(default)]
    pub prefix: Option<String>,
    /// Suffix bytes wrapper.
    #[serde(default)]
    pub suffix: Option<String>,
}

/// Outcome from a full file upload flow.
#[derive(Clone, Debug)]
pub struct FileUploadResult {
    /// True when Zotero reports the same file already exists.
    pub already_exists: bool,
    /// Response metadata from the final upload-related API call.
    pub metadata: crate::responses::response_metadata::ResponseMetadata,
}
