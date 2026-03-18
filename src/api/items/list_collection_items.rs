//! `GET /{libraryScope}/collections/{collectionKey}/items`

use crate::client::{encode_path_segment, ZoteroClient, ZoteroClientError};
use crate::requests::list_items_request::ListItemsRequest;
use crate::responses::paginated_response::PaginatedResponse;
use crate::types::library_scope::LibraryScope;
use crate::types::zotero_object::ZoteroObject;

impl ZoteroClient {
    /// Lists items within a specific collection.
    pub async fn list_collection_items(
        &self,
        scope: LibraryScope,
        collection_key: &str,
        request: &ListItemsRequest,
    ) -> Result<PaginatedResponse<ZoteroObject>, ZoteroClientError> {
        let collection_key = encode_path_segment(collection_key);
        let path = format!("{}/collections/{collection_key}/items", scope.path_prefix());
        self.get_paginated(&path, &request.to_query_pairs(), request.since)
            .await
    }
}
