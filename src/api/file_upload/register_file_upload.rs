//! `POST /{libraryScope}/items/{itemKey}/file` (registration step)

use serde_json::json;

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Registers previously uploaded binary using `upload` key.
    pub async fn register_file_upload(
        &self,
        scope: LibraryScope,
        item_key: &str,
        upload_key: &str,
        write_options: &WriteOptions,
    ) -> Result<(serde_json::Value, ResponseMetadata), ZoteroClientError> {
        let item_key = encode_path_segment(item_key);
        let path = format!("{}/items/{item_key}/file", scope.path_prefix());
        let body = json!({ "upload": upload_key });
        self.post_json(&path, &[], &body, write_options).await
    }
}
