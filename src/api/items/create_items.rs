//! `POST /{libraryScope}/items`

use serde_json::Value;

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Creates one or more items.
    pub async fn create_items(
        &self,
        scope: LibraryScope,
        items: &[Value],
        write_options: &WriteOptions,
    ) -> Result<(Value, ResponseMetadata), ZoteroClientError> {
        let path = format!("{}/items", scope.path_prefix());
        self.post_json(&path, &[], &items, write_options).await
    }
}
