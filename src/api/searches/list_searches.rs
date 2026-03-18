//! `GET /{libraryScope}/searches`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::list_searches_request::ListSearchesRequest;
use crate::responses::paginated_response::PaginatedResponse;
use crate::types::library_scope::LibraryScope;
use crate::types::search::Search;

impl ZoteroClient {
    /// Lists saved searches for a library scope.
    pub async fn list_searches(
        &self,
        scope: LibraryScope,
        request: &ListSearchesRequest,
    ) -> Result<PaginatedResponse<Search>, ZoteroClientError> {
        let path = format!("{}/searches", scope.path_prefix());
        self.get_paginated(&path, &request.to_query_pairs(), request.since)
            .await
    }
}
