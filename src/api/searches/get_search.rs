//! `GET /{libraryScope}/searches/{searchKey}`

use crate::client::encode_path_segment;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::responses::response_metadata::ResponseMetadata;
use crate::types::library_scope::LibraryScope;
use crate::types::search::Search;

impl ZoteroClient {
    /// Retrieves a single saved search by key.
    pub async fn get_search(
        &self,
        scope: LibraryScope,
        search_key: &str,
        if_modified_since_version: Option<u64>,
    ) -> Result<(Search, ResponseMetadata), ZoteroClientError> {
        let search_key = encode_path_segment(search_key);
        let path = format!("{}/searches/{search_key}", scope.path_prefix());
        self.get_json(&path, &[], if_modified_since_version).await
    }
}
