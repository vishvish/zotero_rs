//! Interprets upload-authorization responses.

use crate::client::ZoteroClientError;
use crate::types::file_upload::FileUploadAuthorizationResponse;

/// Next action to take after file-upload authorization.
#[derive(Clone, Debug)]
pub enum FileUploadAction {
    /// No upload needed because the same file already exists.
    AlreadyExists,
    /// Upload binary then register with Zotero.
    UploadRequired {
        /// Upload key returned by Zotero.
        upload_key: String,
        /// Target URL to receive the multipart-like byte payload.
        upload_url: String,
        /// Prefix wrapper bytes, UTF-8 encoded.
        prefix: String,
        /// Suffix wrapper bytes, UTF-8 encoded.
        suffix: String,
    },
}

/// Determines the required next upload step from the authorization response.
pub fn determine_upload_action(
    response: &FileUploadAuthorizationResponse,
) -> Result<FileUploadAction, ZoteroClientError> {
    if response.exists == 1 {
        return Ok(FileUploadAction::AlreadyExists);
    }

    let upload_key = response
        .upload_key
        .as_ref()
        .ok_or_else(|| ZoteroClientError::InvalidFileUploadResponse("upload_key".into()))?;
    let upload_url = response
        .url
        .as_ref()
        .ok_or_else(|| ZoteroClientError::InvalidFileUploadResponse("url".into()))?;
    let prefix = response
        .prefix
        .as_ref()
        .ok_or_else(|| ZoteroClientError::InvalidFileUploadResponse("prefix".into()))?;
    let suffix = response
        .suffix
        .as_ref()
        .ok_or_else(|| ZoteroClientError::InvalidFileUploadResponse("suffix".into()))?;

    Ok(FileUploadAction::UploadRequired {
        upload_key: upload_key.clone(),
        upload_url: upload_url.clone(),
        prefix: prefix.clone(),
        suffix: suffix.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::api::file_upload::determine_upload_action::{
        determine_upload_action, FileUploadAction,
    };
    use crate::types::file_upload::FileUploadAuthorizationResponse;

    #[test]
    fn returns_already_exists_when_exists_is_one() {
        let response = FileUploadAuthorizationResponse {
            exists: 1,
            upload_key: None,
            url: None,
            prefix: None,
            suffix: None,
        };

        let action = determine_upload_action(&response).expect("valid");
        assert!(matches!(action, FileUploadAction::AlreadyExists));
    }

    #[test]
    fn fails_when_upload_required_but_missing_fields() {
        let response = FileUploadAuthorizationResponse {
            exists: 0,
            upload_key: Some("ukey".to_owned()),
            url: None,
            prefix: Some("p".to_owned()),
            suffix: Some("s".to_owned()),
        };

        let err = determine_upload_action(&response).expect_err("must fail");
        assert!(format!("{err}").contains("missing file upload field"));
    }

    #[test]
    fn returns_upload_required_when_all_fields_present() {
        let response = FileUploadAuthorizationResponse {
            exists: 0,
            upload_key: Some("u".to_owned()),
            url: Some("https://example.com/upload".to_owned()),
            prefix: Some("p".to_owned()),
            suffix: Some("s".to_owned()),
        };

        let action = determine_upload_action(&response).expect("must parse");
        assert!(matches!(action, FileUploadAction::UploadRequired { .. }));
    }
}
