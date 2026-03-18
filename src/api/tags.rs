//! Tag endpoint operations.

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::list_tags_request::ListTagsRequest;
use crate::responses::paginated_response::PaginatedResponse;
use crate::types::library_scope::LibraryScope;
use crate::types::tag::Tag;

impl ZoteroClient {
    /// Lists tags for a library scope.
    pub async fn list_tags(
        &self,
        scope: LibraryScope,
        request: &ListTagsRequest,
    ) -> Result<PaginatedResponse<Tag>, ZoteroClientError> {
        let path = format!("{}/tags", scope.path_prefix());
        self.get_paginated(&path, &request.to_query_pairs(), request.since)
            .await
    }
}
