//! `POST /{libraryScope}/collections`

use serde_json::Value;

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Creates one or more collections.
    pub async fn create_collections(
        &self,
        scope: LibraryScope,
        collections: &[Value],
        write_options: &WriteOptions,
    ) -> Result<(Value, ResponseMetadata), ZoteroClientError> {
        let path = format!("{}/collections", scope.path_prefix());
        self.post_json(&path, &[], &collections, write_options)
            .await
    }
}
