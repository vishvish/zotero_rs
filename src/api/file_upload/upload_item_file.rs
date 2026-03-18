//! High-level file upload orchestration.

use crate::api::file_upload::determine_upload_action::{determine_upload_action, FileUploadAction};
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::types::file_upload::{FileUploadAuthorizationRequest, FileUploadResult};
use crate::types::library_scope::LibraryScope;

impl ZoteroClient {
    /// Runs full file-upload flow: authorize -> binary upload -> register.
    pub async fn upload_item_file(
        &self,
        scope: LibraryScope,
        item_key: &str,
        authorization_request: &FileUploadAuthorizationRequest,
        file_bytes: &[u8],
        write_options: &WriteOptions,
        content_type: Option<&str>,
    ) -> Result<FileUploadResult, ZoteroClientError> {
        let (authorization_response, metadata) = self
            .authorize_file_upload(scope, item_key, authorization_request, write_options)
            .await?;

        match determine_upload_action(&authorization_response)? {
            FileUploadAction::AlreadyExists => Ok(FileUploadResult {
                already_exists: true,
                metadata,
            }),
            FileUploadAction::UploadRequired {
                upload_key,
                upload_url,
                prefix,
                suffix,
            } => {
                self.upload_file_binary(&upload_url, &prefix, file_bytes, &suffix, content_type)
                    .await?;

                let (_, register_metadata) = self
                    .register_file_upload(scope, item_key, &upload_key, write_options)
                    .await?;

                Ok(FileUploadResult {
                    already_exists: false,
                    metadata: register_metadata,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;
    use wiremock::matchers::{body_partial_json, body_string, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::client::{ClientOptions, RetryPolicy, ZoteroClient};
    use crate::requests::write_options::WriteOptions;
    use crate::types::file_upload::FileUploadAuthorizationRequest;
    use crate::types::library_scope::LibraryScope;

    #[tokio::test]
    async fn upload_item_file_function_exists() {
        let client = ZoteroClient::new(ClientOptions::default()).expect("client");
        let _ = client;
    }

    fn client_for(server: &MockServer) -> ZoteroClient {
        ZoteroClient::new(ClientOptions {
            base_url: format!("{}/", server.uri()),
            retry_policy: RetryPolicy {
                max_attempts: 2,
                base_delay: Duration::from_millis(1),
                max_delay: Duration::from_millis(2),
            },
            ..ClientOptions::default()
        })
        .expect("client")
    }

    #[tokio::test]
    async fn short_circuits_when_file_already_exists() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/users/1/items/ITEM/file"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "exists": 1 })))
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client
            .upload_item_file(
                LibraryScope::User(1),
                "ITEM",
                &FileUploadAuthorizationRequest {
                    filename: "a.txt".to_owned(),
                    filesize: 4,
                    md5: "deadbeef".to_owned(),
                    mtime: 1,
                    content_type: None,
                    charset: None,
                },
                b"DATA",
                &WriteOptions::default(),
                Some("text/plain"),
            )
            .await
            .expect("must succeed");

        assert!(result.already_exists);
    }

    #[tokio::test]
    async fn runs_full_authorize_upload_register_flow() {
        let server = MockServer::start().await;
        let upload_url = format!("{}/upload-target", server.uri());

        Mock::given(method("POST"))
            .and(path("/users/1/items/ITEM/file"))
            .and(body_partial_json(json!({"filename": "a.txt"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "exists": 0,
                "upload_key": "ukey1",
                "url": upload_url,
                "prefix": "pre",
                "suffix": "suf"
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/upload-target"))
            .and(body_string("preDATAsuf"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/users/1/items/ITEM/file"))
            .and(body_partial_json(json!({"upload": "ukey1"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "registered": true })))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client
            .upload_item_file(
                LibraryScope::User(1),
                "ITEM",
                &FileUploadAuthorizationRequest {
                    filename: "a.txt".to_owned(),
                    filesize: 4,
                    md5: "deadbeef".to_owned(),
                    mtime: 1,
                    content_type: None,
                    charset: None,
                },
                b"DATA",
                &WriteOptions::default(),
                Some("text/plain"),
            )
            .await
            .expect("must succeed");

        assert!(!result.already_exists);
    }
}
