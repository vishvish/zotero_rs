//! Pagination helper methods.

#[cfg(test)]
mod tests {
    use crate::responses::paginated_response::PaginatedResponse;
    use crate::responses::pagination_links::PaginationLinks;
    use crate::responses::response_metadata::ResponseMetadata;

    #[test]
    fn detects_next_page() {
        let metadata = ResponseMetadata {
            links: PaginationLinks {
                next: Some("https://api.zotero.org/users/1/items?start=25".to_owned()),
                ..PaginationLinks::default()
            },
            ..ResponseMetadata::default()
        };

        let page = PaginatedResponse::<u32> {
            data: vec![1, 2],
            metadata,
        };

        assert!(page.has_next_page());
        assert_eq!(
            page.next_page_url(),
            Some("https://api.zotero.org/users/1/items?start=25")
        );
    }
}
