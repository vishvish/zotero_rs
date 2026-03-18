//! `PATCH /{libraryScope}/items/{itemKey}`

use serde_json::Value;

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Updates a single item by key.
    pub async fn update_item(
        &self,
        scope: LibraryScope,
        item_key: &str,
        item: &Value,
        write_options: &WriteOptions,
    ) -> Result<(Value, ResponseMetadata), ZoteroClientError> {
        let item_key = encode_path_segment(item_key);
        let path = format!("{}/items/{item_key}", scope.path_prefix());
        self.patch_json(&path, &[], item, write_options).await
    }
}
