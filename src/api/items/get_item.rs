//! `GET /{libraryScope}/items/{itemKey}`

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;
use crate::types::zotero_object::ZoteroObject;

impl ZoteroClient {
    /// Retrieves a single item by key.
    pub async fn get_item(
        &self,
        scope: LibraryScope,
        item_key: &str,
        if_modified_since_version: Option<u64>,
    ) -> Result<(ZoteroObject, ResponseMetadata), ZoteroClientError> {
        let item_key = encode_path_segment(item_key);
        let path = format!("{}/items/{item_key}", scope.path_prefix());
        self.get_json(&path, &[], if_modified_since_version).await
    }
}
