//! `POST /{libraryScope}/items/{itemKey}/file` (authorization step)

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::file_upload::{FileUploadAuthorizationRequest, FileUploadAuthorizationResponse};
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Authorizes an item file upload and returns upload instructions.
    pub async fn authorize_file_upload(
        &self,
        scope: LibraryScope,
        item_key: &str,
        request: &FileUploadAuthorizationRequest,
        write_options: &WriteOptions,
    ) -> Result<(FileUploadAuthorizationResponse, ResponseMetadata), ZoteroClientError> {
        let item_key = encode_path_segment(item_key);
        let path = format!("{}/items/{item_key}/file", scope.path_prefix());
        self.post_json(&path, &[], request, write_options).await
    }
}
