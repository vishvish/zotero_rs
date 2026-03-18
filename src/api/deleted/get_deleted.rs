//! `GET /{libraryScope}/deleted`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::deleted::DeletedResponse;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Returns keys of recently deleted objects.
    pub async fn get_deleted(
        &self,
        scope: LibraryScope,
        since: u64,
    ) -> Result<(DeletedResponse, ResponseMetadata), ZoteroClientError> {
        let path = format!("{}/deleted", scope.path_prefix());
        let query = vec![("since".to_owned(), since.to_string())];
        self.get_json(&path, &query, None).await
    }
}
