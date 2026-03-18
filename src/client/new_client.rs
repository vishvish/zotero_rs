//! Client constructor and secure defaults.

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::ClientBuilder;
use url::Url;

use crate::client::ZoteroClient;
use crate::client::{ClientOptions, ZoteroClientError};

impl ZoteroClient {
    /// Creates a client with secure defaults.
    pub fn new(options: ClientOptions) -> Result<Self, ZoteroClientError> {
        let base_url = Url::parse(&options.base_url)?;
        validate_base_url(&base_url)?;

        let mut default_headers = HeaderMap::new();
        default_headers.insert("Zotero-API-Version", HeaderValue::from_static("3"));
        default_headers.insert(USER_AGENT, HeaderValue::from_str(&options.user_agent)?);

        let http = ClientBuilder::new()
            .timeout(options.timeout)
            .connect_timeout(options.connect_timeout)
            .redirect(reqwest::redirect::Policy::limited(5))
            .default_headers(default_headers)
            .build()?;

        Ok(Self {
            http,
            base_url,
            options,
        })
    }

    /// Returns the client base URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

fn validate_base_url(base_url: &Url) -> Result<(), ZoteroClientError> {
    if base_url.scheme() != "https" {
        #[cfg(test)]
        if base_url.scheme() == "http"
            && matches!(base_url.host_str(), Some("localhost" | "127.0.0.1"))
        {
            return Ok(());
        }

        return Err(ZoteroClientError::InsecureBaseUrl);
    }

    let Some(host) = base_url.host_str() else {
        return Err(ZoteroClientError::UnsupportedHost);
    };

    if host != "api.zotero.org" {
        #[cfg(test)]
        if host == "localhost" || host == "127.0.0.1" {
            return Ok(());
        }

        return Err(ZoteroClientError::UnsupportedHost);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_http_base_url() {
        let options = ClientOptions {
            base_url: "http://api.zotero.org".to_owned(),
            ..ClientOptions::default()
        };

        let result = ZoteroClient::new(options);
        assert!(matches!(result, Err(ZoteroClientError::InsecureBaseUrl)));
    }

    #[test]
    fn rejects_non_zotero_host() {
        let options = ClientOptions {
            base_url: "https://example.com".to_owned(),
            ..ClientOptions::default()
        };

        let result = ZoteroClient::new(options);
        assert!(matches!(result, Err(ZoteroClientError::UnsupportedHost)));
    }
}
