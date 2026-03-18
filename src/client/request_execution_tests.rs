//! Integration-style tests for request execution behavior.

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

    use crate::client::{ClientOptions, RetryPolicy, ZoteroClient, ZoteroClientError};
    use crate::requests::list_items_request::ListItemsRequest;
    use crate::requests::write_options::WriteOptions;
    use crate::types::library_scope::LibraryScope;

    struct SequenceResponder {
        templates: Vec<ResponseTemplate>,
        index: Arc<AtomicUsize>,
    }

    impl SequenceResponder {
        fn new(templates: Vec<ResponseTemplate>) -> Self {
            Self {
                templates,
                index: Arc::new(AtomicUsize::new(0)),
            }
        }
    }

    impl Respond for SequenceResponder {
        fn respond(&self, _request: &Request) -> ResponseTemplate {
            let i = self.index.fetch_add(1, Ordering::SeqCst);
            self.templates.get(i).cloned().unwrap_or_else(|| {
                self.templates
                    .last()
                    .cloned()
                    .expect("at least one template")
            })
        }
    }

    fn make_client(server: &MockServer, attempts: u32) -> ZoteroClient {
        ZoteroClient::new(ClientOptions {
            base_url: format!("{}/", server.uri()),
            retry_policy: RetryPolicy {
                max_attempts: attempts,
                base_delay: Duration::from_millis(1),
                max_delay: Duration::from_millis(3),
            },
            ..ClientOptions::default()
        })
        .expect("client")
    }

    #[tokio::test]
    async fn retries_safe_get_after_503() {
        let server = MockServer::start().await;
        let route = "/users/1/items";

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(SequenceResponder::new(vec![
                ResponseTemplate::new(503).insert_header("Retry-After", "0"),
                ResponseTemplate::new(200).set_body_json(json!([])),
            ]))
            .mount(&server)
            .await;

        let client = make_client(&server, 2);
        let out = client
            .list_items(LibraryScope::User(1), &ListItemsRequest::default())
            .await
            .expect("must succeed after retry");

        assert!(out.data.is_empty());
    }

    #[tokio::test]
    async fn does_not_retry_post_by_default() {
        let server = MockServer::start().await;
        let route = "/users/1/items";

        Mock::given(method("POST"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&server)
            .await;

        let client = make_client(&server, 2);
        let err = client
            .create_items(LibraryScope::User(1), &[], &WriteOptions::default())
            .await
            .expect_err("post should not retry by default");

        assert!(matches!(err, ZoteroClientError::ServiceUnavailable { .. }));
    }

    #[tokio::test]
    async fn fetches_next_page_from_link_header() {
        let server = MockServer::start().await;
        let route = "/users/1/items";
        let next_url = format!("{}/users/1/items?start=1", server.uri());

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(SequenceResponder::new(vec![
                ResponseTemplate::new(200)
                    .insert_header("Link", format!("<{next_url}>; rel=\"next\""))
                    .set_body_json(json!([])),
                ResponseTemplate::new(200).set_body_json(json!([
                    {
                        "key": "ABCD1234",
                        "version": 1,
                        "data": {"title": "hello"}
                    }
                ])),
            ]))
            .mount(&server)
            .await;

        let client = make_client(&server, 2);
        let first = client
            .list_items(LibraryScope::User(1), &ListItemsRequest::default())
            .await
            .expect("first page");

        let second = client
            .fetch_next_page(&first)
            .await
            .expect("fetch next")
            .expect("has next");

        assert_eq!(second.data.len(), 1);
        assert_eq!(second.data[0].key, "ABCD1234");
    }

    #[tokio::test]
    async fn lists_items_within_specific_collection_endpoint() {
        let server = MockServer::start().await;
        let route = "/users/1/collections/ABCD1234/items";

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .expect(1)
            .mount(&server)
            .await;

        let client = make_client(&server, 1);
        let out = client
            .list_collection_items(
                LibraryScope::User(1),
                "ABCD1234",
                &ListItemsRequest::default(),
            )
            .await
            .expect("must list items in collection");

        assert!(out.data.is_empty());
    }

    #[tokio::test]
    async fn lists_top_level_items_within_specific_collection_endpoint() {
        let server = MockServer::start().await;
        let route = "/users/1/collections/ABCD1234/items/top";

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .expect(1)
            .mount(&server)
            .await;

        let client = make_client(&server, 1);
        let out = client
            .list_collection_top_items(
                LibraryScope::User(1),
                "ABCD1234",
                &ListItemsRequest::default(),
            )
            .await
            .expect("must list top-level items in collection");

        assert!(out.data.is_empty());
    }
}
