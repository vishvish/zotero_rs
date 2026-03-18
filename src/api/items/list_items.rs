//! `GET /{libraryScope}/items`

use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::list_items_request::ListItemsRequest;
use crate::responses::paginated_response::PaginatedResponse;
use crate::types::library_scope::LibraryScope;
use crate::types::zotero_object::ZoteroObject;

impl ZoteroClient {
    /// Lists items for a library scope.
    pub async fn list_items(
        &self,
        scope: LibraryScope,
        request: &ListItemsRequest,
    ) -> Result<PaginatedResponse<ZoteroObject>, ZoteroClientError> {
        let path = format!("{}/items", scope.path_prefix());
        self.get_paginated(&path, &request.to_query_pairs(), request.since)
            .await
    }
}
