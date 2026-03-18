//! Structured item metadata extracted from Zotero item payloads.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::zotero_object::ZoteroObject;

/// A creator entry from Zotero item data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ItemCreator {
    /// Creator role (e.g., `author`, `editor`).
    pub creator_type: Option<String>,
    /// First name when present.
    pub first_name: Option<String>,
    /// Last name when present.
    pub last_name: Option<String>,
    /// Single-field name used by some creator entries.
    pub name: Option<String>,
}

/// Structured item metadata with common fields and full raw payload.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ItemMetadata {
    /// Item key.
    pub key: String,
    /// Item version.
    pub version: u64,
    /// Zotero item type (e.g., `webpage`, `blogPost`).
    pub item_type: Option<String>,
    /// Item title.
    pub title: Option<String>,
    /// Item URL when available.
    pub url: Option<String>,
    /// Item abstract (`abstractNote`) when available.
    pub abstract_note: Option<String>,
    /// All creators attached to the item.
    pub creators: Vec<ItemCreator>,
    /// Author display names, extracted from creators with `creatorType=author`.
    pub authors: Vec<String>,
    /// Full raw Zotero `data` payload to preserve all metadata fields.
    pub raw_data: Value,
}

impl From<&ZoteroObject> for ItemMetadata {
    fn from(item: &ZoteroObject) -> Self {
        let creators = creators_from_data(&item.data);
        let authors = creators
            .iter()
            .filter(|creator| creator.creator_type.as_deref() == Some("author"))
            .filter_map(creator_display_name)
            .collect::<Vec<_>>();

        Self {
            key: item.key.clone(),
            version: item.version,
            item_type: string_field(&item.data, "itemType"),
            title: string_field(&item.data, "title"),
            url: string_field(&item.data, "url"),
            abstract_note: string_field(&item.data, "abstractNote"),
            creators,
            authors,
            raw_data: item.data.clone(),
        }
    }
}

fn string_field(data: &Value, field: &str) -> Option<String> {
    data.get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn creators_from_data(data: &Value) -> Vec<ItemCreator> {
    data.get("creators")
        .and_then(Value::as_array)
        .map(|creators| {
            creators
                .iter()
                .map(|creator| ItemCreator {
                    creator_type: creator
                        .get("creatorType")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    first_name: creator
                        .get("firstName")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    last_name: creator
                        .get("lastName")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    name: creator
                        .get("name")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn creator_display_name(creator: &ItemCreator) -> Option<String> {
    if let Some(name) = creator.name.as_ref().filter(|name| !name.is_empty()) {
        return Some(name.clone());
    }

    match (&creator.first_name, &creator.last_name) {
        (Some(first), Some(last)) => Some(format!("{first} {last}")),
        (Some(first), None) => Some(first.clone()),
        (None, Some(last)) => Some(last.clone()),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::zotero_object::ZoteroObject;

    #[test]
    fn extracts_structured_fields_from_item_data() {
        let object = ZoteroObject {
            key: "ABCD1234".to_owned(),
            version: 42,
            data: json!({
                "itemType": "journalArticle",
                "title": "Typed metadata for Zotero",
                "url": "https://example.com/paper",
                "abstractNote": "A short abstract.",
                "creators": [
                    {"creatorType": "author", "firstName": "Ada", "lastName": "Lovelace"},
                    {"creatorType": "author", "name": "Grace Hopper"},
                    {"creatorType": "editor", "firstName": "Ed", "lastName": "Itor"}
                ],
                "DOI": "10.0000/example"
            }),
        };

        let metadata = object.to_item_metadata();
        assert_eq!(metadata.key, "ABCD1234");
        assert_eq!(metadata.version, 42);
        assert_eq!(metadata.item_type.as_deref(), Some("journalArticle"));
        assert_eq!(metadata.title.as_deref(), Some("Typed metadata for Zotero"));
        assert_eq!(metadata.url.as_deref(), Some("https://example.com/paper"));
        assert_eq!(metadata.abstract_note.as_deref(), Some("A short abstract."));
        assert_eq!(metadata.authors, vec!["Ada Lovelace", "Grace Hopper"]);
        assert_eq!(metadata.creators.len(), 3);
        assert_eq!(metadata.raw_data["DOI"], "10.0000/example");
    }

    #[test]
    fn handles_missing_fields_without_failing() {
        let object = ZoteroObject {
            key: "NOFIELDS".to_owned(),
            version: 1,
            data: json!({}),
        };

        let metadata = object.to_item_metadata();
        assert_eq!(metadata.item_type, None);
        assert_eq!(metadata.title, None);
        assert_eq!(metadata.url, None);
        assert_eq!(metadata.abstract_note, None);
        assert!(metadata.authors.is_empty());
        assert!(metadata.creators.is_empty());
    }
}
