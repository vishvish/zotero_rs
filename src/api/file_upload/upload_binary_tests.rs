//! Tests for upload URL validation.

#[cfg(test)]
mod tests {
    use crate::client::{ClientOptions, ZoteroClient};

    #[tokio::test]
    async fn rejects_non_https_upload_url() {
        let client = ZoteroClient::new(ClientOptions::default()).expect("client");
        let err = client
            .upload_file_binary("http://example.com/upload", "", b"a", "", None)
            .await
            .expect_err("must fail");

        assert!(format!("{err}").contains("upload URL must use https"));
    }
}
