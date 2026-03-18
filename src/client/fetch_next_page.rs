//! Helper for following paginated `rel=next` links.

use serde::de::DeserializeOwned;

use crate::client::auth::apply_auth;
use crate::client::headers::parse_response_metadata;
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::responses::paginated_response::PaginatedResponse;

impl ZoteroClient {
    /// Fetches the next page using `Link: rel="next"` from a previous page.
    pub async fn fetch_next_page<T: DeserializeOwned>(
        &self,
        current_page: &PaginatedResponse<T>,
    ) -> Result<Option<PaginatedResponse<T>>, ZoteroClientError> {
        let Some(next_url) = current_page.next_page_url() else {
            return Ok(None);
        };

        let next = url::Url::parse(next_url)?;
        if next.scheme() != self.base_url.scheme() || next.host_str() != self.base_url.host_str() {
            return Err(ZoteroClientError::UnsupportedHost);
        }

        let request = apply_auth(self.http.get(next), &self.options.auth);
        let response = self.execute(request, true).await?;
        let metadata = parse_response_metadata(response.headers());
        let data = response.json::<Vec<T>>().await?;

        Ok(Some(PaginatedResponse { data, metadata }))
    }
}
