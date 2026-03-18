//! Binary upload to Zotero-provided upload URL.

use crate::api::file_upload::compose_upload_body::compose_upload_body;
use crate::client::{ZoteroClient, ZoteroClientError};

impl ZoteroClient {
    /// Uploads file bytes to the provided upload URL.
    pub async fn upload_file_binary(
        &self,
        upload_url: &str,
        prefix: &str,
        file_bytes: &[u8],
        suffix: &str,
        content_type: Option<&str>,
    ) -> Result<(), ZoteroClientError> {
        let parsed = url::Url::parse(upload_url)?;
        if parsed.scheme() != "https" {
            #[cfg(test)]
            if matches!(parsed.host_str(), Some("localhost" | "127.0.0.1")) {
                // Allowed in test builds for local mocked upload flows.
            } else {
                return Err(ZoteroClientError::InsecureUploadUrl);
            }

            #[cfg(not(test))]
            return Err(ZoteroClientError::InsecureUploadUrl);
        }

        let body = compose_upload_body(prefix, file_bytes, suffix);

        let mut request = self.http.post(parsed).body(body);
        if let Some(value) = content_type {
            request = request.header("Content-Type", value);
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(ZoteroClientError::HttpStatus {
                status: response.status(),
                body: response.text().await.unwrap_or_default().into_boxed_str(),
                metadata: Box::default(),
            })
        }
    }
}
