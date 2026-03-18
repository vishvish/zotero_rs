//! Helpers for building file-upload payload bytes.

/// Builds upload body bytes from Zotero-provided `prefix` and `suffix` wrappers.
pub fn compose_upload_body(prefix: &str, file_bytes: &[u8], suffix: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(prefix.len() + file_bytes.len() + suffix.len());
    out.extend_from_slice(prefix.as_bytes());
    out.extend_from_slice(file_bytes);
    out.extend_from_slice(suffix.as_bytes());
    out
}

#[cfg(test)]
mod tests {
    use crate::api::file_upload::compose_upload_body::compose_upload_body;

    #[test]
    fn composes_prefix_file_suffix() {
        let out = compose_upload_body("abc", b"DATA", "xyz");
        assert_eq!(out, b"abcDATAxyz");
    }
}
