use secrecy::SecretString;
use zotero_rs::requests::list_items_request::ListItemsRequest;
use zotero_rs::{Auth, ClientOptions, LibraryScope, ZoteroClient};

#[tokio::test]
#[ignore = "requires real Zotero credentials and network"]
async fn live_list_items_smoke() {
    if std::env::var("ZOTERO_LIVE_TESTS").ok().as_deref() != Some("1") {
        return;
    }

    let api_key = std::env::var("ZOTERO_API_KEY").expect("ZOTERO_API_KEY must be set");
    let user_id: u64 = std::env::var("ZOTERO_USER_ID")
        .expect("ZOTERO_USER_ID must be set")
        .parse()
        .expect("ZOTERO_USER_ID must parse as u64");

    let client = ZoteroClient::new(ClientOptions {
        auth: Some(Auth::ApiKey(SecretString::from(api_key))),
        ..ClientOptions::default()
    })
    .expect("client");

    let page = client
        .list_items(
            LibraryScope::User(user_id),
            &ListItemsRequest {
                limit: Some(1),
                ..ListItemsRequest::default()
            },
        )
        .await
        .expect("live list_items should succeed");

    assert!(page.data.len() <= 1);
}
