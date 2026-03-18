//! `GET /{libraryScope}/collections`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::list_collections_request::ListCollectionsRequest;
use crate::responses::paginated_response::PaginatedResponse;
use crate::types::collection::Collection;
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Lists collections for a library scope.
    pub async fn list_collections(
        &self,
        scope: LibraryScope,
        request: &ListCollectionsRequest,
    ) -> Result<PaginatedResponse<Collection>, ZoteroClientError> {
        let path = format!("{}/collections", scope.path_prefix());
        self.get_paginated(&path, &request.to_query_pairs(), request.since)
            .await
    }
}
