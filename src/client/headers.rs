//! HTTP header parsing helpers.

use reqwest::header::HeaderMap;

use crate::responses::pagination_links::PaginationLinks;
use crate::responses::response_metadata::ResponseMetadata;

pub(crate) fn parse_response_metadata(headers: &HeaderMap) -> ResponseMetadata {
    ResponseMetadata {
        last_modified_version: parse_u64_header(headers, "Last-Modified-Version"),
        library_version: parse_u64_header(headers, "Zotero-Library-Version"),
        backoff_seconds: parse_u64_header(headers, "Backoff"),
        retry_after_seconds: parse_u64_header(headers, "Retry-After"),
        links: parse_link_header(headers),
    }
}

fn parse_u64_header(headers: &HeaderMap, name: &str) -> Option<u64> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
}

fn parse_link_header(headers: &HeaderMap) -> PaginationLinks {
    let mut links = PaginationLinks::default();

    let Some(value) = headers.get("Link").and_then(|header| header.to_str().ok()) else {
        return links;
    };

    for segment in value.split(',') {
        let mut parts = segment.split(';').map(|part| part.trim());
        let Some(url_part) = parts.next() else {
            continue;
        };

        let url = url_part
            .strip_prefix('<')
            .and_then(|part| part.strip_suffix('>'));

        let relation = parts
            .find(|part| part.starts_with("rel="))
            .map(|part| part.trim_start_matches("rel=").trim_matches('"'));

        let (Some(url), Some(rel)) = (url, relation) else {
            continue;
        };

        match rel {
            "first" => links.first = Some(url.to_owned()),
            "prev" => links.prev = Some(url.to_owned()),
            "next" => links.next = Some(url.to_owned()),
            "last" => links.last = Some(url.to_owned()),
            "alternate" => links.alternate = Some(url.to_owned()),
            _ => {}
        }
    }

    links
}

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue};

    use super::parse_response_metadata;

    #[test]
    fn parses_metadata_and_link_relations() {
        let mut headers = HeaderMap::new();
        headers.insert("Last-Modified-Version", HeaderValue::from_static("14"));
        headers.insert("Zotero-Library-Version", HeaderValue::from_static("15"));
        headers.insert("Backoff", HeaderValue::from_static("30"));
        headers.insert("Retry-After", HeaderValue::from_static("12"));
        headers.insert(
            "Link",
            HeaderValue::from_static(
                "<https://api.zotero.org/users/1/items?start=0>; rel=\"first\", <https://api.zotero.org/users/1/items?start=25>; rel=\"next\"",
            ),
        );

        let metadata = parse_response_metadata(&headers);
        assert_eq!(metadata.last_modified_version, Some(14));
        assert_eq!(metadata.library_version, Some(15));
        assert_eq!(metadata.backoff_seconds, Some(30));
        assert_eq!(metadata.retry_after_seconds, Some(12));
        assert_eq!(
            metadata.links.first.as_deref(),
            Some("https://api.zotero.org/users/1/items?start=0")
        );
        assert_eq!(
            metadata.links.next.as_deref(),
            Some("https://api.zotero.org/users/1/items?start=25")
        );
    }
}
