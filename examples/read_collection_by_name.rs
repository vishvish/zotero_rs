use secrecy::SecretString;
use zotero_rs::requests::list_collections_request::ListCollectionsRequest;
use zotero_rs::requests::list_items_request::ListItemsRequest;
use zotero_rs::types::item_metadata::ItemMetadata;
use zotero_rs::{Auth, ClientOptions, LibraryScope, ZoteroClient};

fn item_field<'a>(item: &'a serde_json::Value, field: &str) -> Option<&'a str> {
    item.get(field).and_then(serde_json::Value::as_str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("ZOTERO_API_KEY")?;
    let user_id: u64 = std::env::var("ZOTERO_USER_ID")?.parse()?;
    let collection_name = std::env::var("ZOTERO_COLLECTION_NAME")
        .map_err(|_| "ZOTERO_COLLECTION_NAME must be set for this example")?;
    let top_level_only = std::env::var("ZOTERO_TOP_LEVEL_ONLY")
        .map(|value| value == "1")
        .unwrap_or(true);
    let include_trashed = std::env::var("ZOTERO_INCLUDE_TRASHED")
        .map(|value| value == "1")
        .unwrap_or(false);

    let client = ZoteroClient::new(ClientOptions {
        auth: Some(Auth::ApiKey(SecretString::from(api_key))),
        ..ClientOptions::default()
    })?;
    let scope = LibraryScope::User(user_id);

    let mut collection_page = client
        .list_collections(
            scope,
            &ListCollectionsRequest {
                limit: Some(100),
                ..ListCollectionsRequest::default()
            },
        )
        .await?;

    let mut collection_key = collection_page.data.iter().find_map(|collection| {
        let name = item_field(&collection.data, "name")?;
        (name == collection_name).then(|| collection.key.clone())
    });

    while collection_key.is_none() {
        let Some(next) = client.fetch_next_page(&collection_page).await? else {
            break;
        };
        collection_page = next;
        collection_key = collection_page.data.iter().find_map(|collection| {
            let name = item_field(&collection.data, "name")?;
            (name == collection_name).then(|| collection.key.clone())
        });
    }

    let Some(collection_key) = collection_key else {
        return Err(format!("collection not found: {collection_name}").into());
    };

    println!(
        "Collection \"{}\" => key {}",
        collection_name, collection_key
    );

    let request = ListItemsRequest {
        limit: Some(100),
        include_trash: include_trashed,
        ..ListItemsRequest::default()
    };

    let mut items_page = if top_level_only {
        client
            .list_collection_top_items(scope, &collection_key, &request)
            .await?
    } else {
        client
            .list_collection_items(scope, &collection_key, &request)
            .await?
    };

    let mut total = 0usize;
    let mut structured_items: Vec<ItemMetadata> = Vec::new();
    println!(
        "Reading {} items in collection (includeTrashed={})",
        if top_level_only { "top-level" } else { "all" },
        include_trashed
    );
    loop {
        for item in &items_page.data {
            let metadata = item.to_item_metadata();
            let item_type = metadata.item_type.as_deref().unwrap_or("unknown");
            let title = metadata.title.as_deref().unwrap_or("(no title)");
            println!(
                "{} v{} [{}] {}",
                metadata.key, metadata.version, item_type, title
            );
            structured_items.push(metadata);
            total += 1;
        }

        let Some(next) = client.fetch_next_page(&items_page).await? else {
            break;
        };
        items_page = next;
    }

    println!("\nStructured item metadata:");
    for metadata in &structured_items {
        println!("{}", serde_json::to_string_pretty(metadata)?);
    }

    println!("Total items read: {}", total);
    Ok(())
}
