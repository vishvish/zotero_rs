//! `PATCH /{libraryScope}/collections/{collectionKey}`

use serde_json::Value;

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Updates a collection by key.
    pub async fn update_collection(
        &self,
        scope: LibraryScope,
        collection_key: &str,
        collection: &Value,
        write_options: &WriteOptions,
    ) -> Result<(Value, ResponseMetadata), ZoteroClientError> {
        let collection_key = encode_path_segment(collection_key);
        let path = format!("{}/collections/{collection_key}", scope.path_prefix());
        self.patch_json(&path, &[], collection, write_options).await
    }
}
