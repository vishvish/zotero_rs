//! Options for write requests.

/// Precondition and idempotency headers for writes.
#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    /// Sends `If-Unmodified-Since-Version` when set.
    pub if_unmodified_since_version: Option<u64>,
    /// Sends `Zotero-Write-Token` when set.
    pub write_token: Option<String>,
}
