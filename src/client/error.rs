//! Error values returned by client operations.

use reqwest::StatusCode;
use thiserror::Error;

use crate::responses::response_metadata::ResponseMetadata;

/// Error type for all client operations.
#[derive(Debug, Error)]
pub enum ZoteroClientError {
    /// Base URL must be HTTPS.
    #[error("base URL must use https")]
    InsecureBaseUrl,
    /// Base URL host must be `api.zotero.org`.
    #[error("base URL host must be api.zotero.org")]
    UnsupportedHost,
    /// URL parsing failed.
    #[error("URL parse failed: {0}")]
    UrlParse(#[from] url::ParseError),
    /// `User-Agent` value is invalid.
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    /// Request execution failed.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    /// OAuth endpoint response could not be parsed.
    #[error("invalid oauth response: {0}")]
    InvalidOAuthResponse(Box<str>),
    /// File-upload response is missing required fields.
    #[error("missing file upload field: {0}")]
    InvalidFileUploadResponse(Box<str>),
    /// Upload URL must use HTTPS.
    #[error("upload URL must use https")]
    InsecureUploadUrl,
    /// API returned 304 Not Modified.
    #[error("resource not modified")]
    NotModified {
        /// Parsed response metadata.
        metadata: Box<ResponseMetadata>,
    },
    /// API returned 412 Precondition Failed.
    #[error("write precondition failed")]
    PreconditionFailed {
        /// Parsed response metadata.
        metadata: Box<ResponseMetadata>,
    },
    /// API returned 429 Too Many Requests.
    #[error("rate limited")]
    RateLimited {
        /// Parsed response metadata.
        metadata: Box<ResponseMetadata>,
    },
    /// API returned 503 Service Unavailable.
    #[error("service unavailable")]
    ServiceUnavailable {
        /// Parsed response metadata.
        metadata: Box<ResponseMetadata>,
    },
    /// API returned non-success status.
    #[error("api returned status {status}")]
    HttpStatus {
        /// HTTP status code.
        status: StatusCode,
        /// Response body text.
        body: Box<str>,
        /// Parsed metadata from headers.
        metadata: Box<ResponseMetadata>,
    },
}
