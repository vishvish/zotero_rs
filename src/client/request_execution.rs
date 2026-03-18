//! Shared request execution methods used by resource operation files.

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::auth::apply_auth;
use crate::client::headers::parse_response_metadata;
use crate::client::retry_policy::{
    compute_retry_delay, should_retry_response, should_retry_transport_error,
};
use crate::client::{ZoteroClient, ZoteroClientError};
use crate::requests::write_options::WriteOptions;
use crate::responses::paginated_response::PaginatedResponse;
use crate::responses::response_metadata::ResponseMetadata;

impl ZoteroClient {
    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        if_modified_since_version: Option<u64>,
    ) -> Result<(T, ResponseMetadata), ZoteroClientError> {
        let mut request = apply_auth(
            self.http.get(self.base_url.join(path)?).query(query),
            &self.options.auth,
        );
        if let Some(version) = if_modified_since_version {
            request = request.header("If-Modified-Since-Version", version.to_string());
        }

        let response = self.execute(request, true).await?;
        let metadata = parse_response_metadata(response.headers());
        let data = response.json::<T>().await?;
        Ok((data, metadata))
    }

    pub(crate) async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        if_modified_since_version: Option<u64>,
    ) -> Result<PaginatedResponse<T>, ZoteroClientError> {
        let (data, metadata) = self
            .get_json::<Vec<T>>(path, query, if_modified_since_version)
            .await?;

        Ok(PaginatedResponse { data, metadata })
    }

    pub(crate) async fn post_json<TBody: Serialize, TResponse: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &TBody,
        write_options: &WriteOptions,
    ) -> Result<(TResponse, ResponseMetadata), ZoteroClientError> {
        let mut request = apply_auth(
            self.http
                .post(self.base_url.join(path)?)
                .query(query)
                .json(body),
            &self.options.auth,
        );

        request = with_write_headers(request, write_options);

        let response = self.execute(request, false).await?;
        let metadata = parse_response_metadata(response.headers());
        let data = response.json::<TResponse>().await?;
        Ok((data, metadata))
    }

    pub(crate) async fn patch_json<TBody: Serialize, TResponse: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &TBody,
        write_options: &WriteOptions,
    ) -> Result<(TResponse, ResponseMetadata), ZoteroClientError> {
        let mut request = apply_auth(
            self.http
                .patch(self.base_url.join(path)?)
                .query(query)
                .json(body),
            &self.options.auth,
        );

        request = with_write_headers(request, write_options);

        let response = self.execute(request, false).await?;
        let metadata = parse_response_metadata(response.headers());
        let data = response.json::<TResponse>().await?;
        Ok((data, metadata))
    }

    pub(crate) async fn delete(
        &self,
        path: &str,
        query: &[(String, String)],
        write_options: &WriteOptions,
    ) -> Result<ResponseMetadata, ZoteroClientError> {
        let mut request = apply_auth(
            self.http.delete(self.base_url.join(path)?).query(query),
            &self.options.auth,
        );
        request = with_write_headers(request, write_options);

        let response = self.execute(request, false).await?;
        Ok(parse_response_metadata(response.headers()))
    }

    pub(crate) async fn execute(
        &self,
        request: reqwest::RequestBuilder,
        is_safe_retry: bool,
    ) -> Result<reqwest::Response, ZoteroClientError> {
        let attempts = self.options.retry_policy.max_attempts.max(1);
        let mut last_error: Option<ZoteroClientError> = None;

        for attempt in 0..attempts {
            let Some(cloned_request) = request.try_clone() else {
                break;
            };

            match cloned_request.send().await {
                Ok(response) => {
                    let status = response.status();
                    let metadata = parse_response_metadata(response.headers());

                    if status.is_success() {
                        return Ok(response);
                    }
                    if status == reqwest::StatusCode::NOT_MODIFIED {
                        return Err(ZoteroClientError::NotModified {
                            metadata: Box::new(metadata),
                        });
                    }
                    if status == reqwest::StatusCode::PRECONDITION_FAILED {
                        return Err(ZoteroClientError::PreconditionFailed {
                            metadata: Box::new(metadata),
                        });
                    }
                    if should_retry_response(is_safe_retry, status) && attempt + 1 < attempts {
                        let delay =
                            compute_retry_delay(attempt, &metadata, self.options.retry_policy);
                        std::thread::sleep(delay);
                        last_error = Some(if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                            ZoteroClientError::RateLimited {
                                metadata: Box::new(metadata),
                            }
                        } else {
                            ZoteroClientError::ServiceUnavailable {
                                metadata: Box::new(metadata),
                            }
                        });
                        continue;
                    }
                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        return Err(ZoteroClientError::RateLimited {
                            metadata: Box::new(metadata),
                        });
                    }
                    if status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
                        return Err(ZoteroClientError::ServiceUnavailable {
                            metadata: Box::new(metadata),
                        });
                    }

                    let body = response.text().await.unwrap_or_default();
                    return Err(ZoteroClientError::HttpStatus {
                        status,
                        body: body.into_boxed_str(),
                        metadata: Box::new(metadata),
                    });
                }
                Err(error) => {
                    if should_retry_transport_error(is_safe_retry) && attempt + 1 < attempts {
                        let delay = compute_retry_delay(
                            attempt,
                            &ResponseMetadata::default(),
                            self.options.retry_policy,
                        );
                        std::thread::sleep(delay);
                        last_error = Some(ZoteroClientError::Request(error));
                        continue;
                    }
                    last_error = Some(ZoteroClientError::Request(error));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| ZoteroClientError::HttpStatus {
            status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            body: "request failed without explicit error"
                .to_owned()
                .into_boxed_str(),
            metadata: Box::new(ResponseMetadata::default()),
        }))
    }
}

fn with_write_headers(
    mut request: reqwest::RequestBuilder,
    write_options: &WriteOptions,
) -> reqwest::RequestBuilder {
    if let Some(version) = write_options.if_unmodified_since_version {
        request = request.header("If-Unmodified-Since-Version", version.to_string());
    }
    if let Some(token) = &write_options.write_token {
        request = request.header("Zotero-Write-Token", token);
    }

    request
}
