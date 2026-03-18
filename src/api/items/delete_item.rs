//! `DELETE /{libraryScope}/items/{itemKey}`

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Deletes an item by key.
    pub async fn delete_item(
        &self,
        scope: LibraryScope,
        item_key: &str,
        write_options: &WriteOptions,
    ) -> Result<ResponseMetadata, ZoteroClientError> {
        let item_key = encode_path_segment(item_key);
        let path = format!("{}/items/{item_key}", scope.path_prefix());
        self.delete(&path, &[], write_options).await
    }
}
