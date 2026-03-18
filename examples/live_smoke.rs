use secrecy::SecretString;
use serde_json::Value;
use zotero_rs::requests::list_items_request::ListItemsRequest;
use zotero_rs::{Auth, ClientOptions, LibraryScope, ZoteroClient};

fn string_field<'a>(data: &'a Value, field: &str) -> Option<&'a str> {
    data.get(field).and_then(Value::as_str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("ZOTERO_API_KEY")?;
    let user_id: u64 = std::env::var("ZOTERO_USER_ID")?.parse()?;

    let client = ZoteroClient::new(ClientOptions {
        auth: Some(Auth::ApiKey(SecretString::from(api_key))),
        ..ClientOptions::default()
    })?;

    let page = client
        .list_items(
            LibraryScope::User(user_id),
            &ListItemsRequest {
                limit: Some(5),
                ..ListItemsRequest::default()
            },
        )
        .await?;

    println!("Fetched {} item(s)", page.data.len());
    for item in page.data {
        let item_type = string_field(&item.data, "itemType").unwrap_or("unknown");
        let title = string_field(&item.data, "title").unwrap_or("(no title)");
        println!("- {} v{} [{}] {}", item.key, item.version, item_type, title);
    }

    Ok(())
}
