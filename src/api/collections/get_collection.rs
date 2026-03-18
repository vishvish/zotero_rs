//! `GET /{libraryScope}/collections/{collectionKey}`

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::collection::Collection;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Retrieves a single collection by key.
    pub async fn get_collection(
        &self,
        scope: LibraryScope,
        collection_key: &str,
        if_modified_since_version: Option<u64>,
    ) -> Result<(Collection, ResponseMetadata), ZoteroClientError> {
        let collection_key = encode_path_segment(collection_key);
        let path = format!("{}/collections/{collection_key}", scope.path_prefix());
        self.get_json(&path, &[], if_modified_since_version).await
    }
}
