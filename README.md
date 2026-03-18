# zotero-rs

A secure, typed Rust client for the Zotero Web API v3.

## Status

Usable, decomposed client with endpoint calls split into operation files.

## Security posture

- HTTPS-only and restricted API host (`api.zotero.org`)
- Default `Zotero-API-Version: 3` and explicit `User-Agent`
- No `unsafe` code in library
- Secret-aware auth types (`secrecy`)
- CI gates for format, clippy, tests, advisories, and license policy
- Path-segment encoding for key-based endpoints
- Conditional read/write headers for Zotero versioning semantics
- Retry policy for safe reads (`GET`) with `Retry-After`/`Backoff` support
- HTTPS-only remote upload targets (localhost HTTP allowed only in test builds)

## Quick start

```rust
use zotero_api_rs::{ClientOptions, LibraryScope, WriteOptions, ZoteroClient};
use zotero_api_rs::requests::list_items_request::ListItemsRequest;

let client = ZoteroClient::new(ClientOptions::default())?;
let scope = LibraryScope::User(12345);

let page = client.list_items(scope, &ListItemsRequest::default()).await?;
let item = client.get_item(scope, "ABCD1234", None).await?;
let _created = client
    .create_items(scope, &[], &WriteOptions::default())
    .await?;
```

## Endpoint organization

API calls are intentionally decomposed so each operation is easy to find:

- `src/api/items/*` for item operations
- `src/api/collections/*` for collection operations
- `src/api/searches/*` for saved search operations
- `src/api/tags/*` for tag operations
- `src/api/deleted/*` for sync deleted-key operations

## Hardening test coverage

- Retry behavior: safe-read retry, write no-retry default, and `rel=next` traversal
- OAuth parsing and signature-base determinism
- File upload action validation and full authorize/upload/register orchestration
- URL/path safety checks for endpoint key segments

## Local secrets

- Keep real API tokens in `.envrc.local` (git-ignored).
- `.envrc` should only source `.envrc.local` and must never contain raw secrets.

## Live account smoke testing (Phase 6)

1. Create a local env file from the template:
```bash
cp .envrc.local.example .envrc.local
```
2. Set real values in `.envrc.local`:
```bash
export ZOTERO_API_KEY="..."
export ZOTERO_USER_ID="1234567"
export ZOTERO_COLLECTION_NAME="example-collection-name"
export ZOTERO_TOP_LEVEL_ONLY="1"
export ZOTERO_INCLUDE_TRASHED="0"
```
3. Run the ignored live integration smoke test:
```bash
set -a; source .envrc.local; set +a
ZOTERO_LIVE_TESTS=1 cargo test --test live_smoke -- --ignored
```
4. Or run the live example:
```bash
set -a; source .envrc.local; set +a
cargo run --example live_smoke
```

Notes:
- These commands are intentionally local-only and are not run in CI.
- Never commit `.envrc.local` or real credentials.

## Examples

- `live_smoke`: fetches first page of items from your library and prints basic item info.
- `read_collection_by_name`: finds a collection by name and prints structured metadata objects for each returned item.

Run:
```bash
set -a; source .envrc.local; set +a
cargo run --example live_smoke
```

```bash
set -a; source .envrc.local; set +a
cargo run --example read_collection_by_name
```

Dummy sample output for `live_smoke`:
```text
Fetched 5 item(s)
- A1B2C3D4 v120 [attachment] Snapshot
- E5F6G7H8 v121 [webpage] Notes on Typed API Clients
- I9J0K1L2 v122 [blogPost] The Geometry of Infinity
- M3N4O5P6 v123 [film] The Medium and the Message
- Q7R8S9T0 v124 [journalArticle] Broad Spectrum Lighting and Vision
```

Dummy sample output for `read_collection_by_name`:
```text
Collection "example-collection-name" => key COL1ECTN
Reading top-level items in collection (includeTrashed=false)
I9J0K1L2 v122 [blogPost] The Geometry of Infinity
M3N4O5P6 v123 [film] The Medium and the Message
Q7R8S9T0 v124 [journalArticle] Broad Spectrum Lighting and Vision
Total items read: 3
```

Dummy sample structured object returned by `read_collection_by_name`:
```json
{
  "key": "Q7R8S9T0",
  "version": 124,
  "item_type": "journalArticle",
  "title": "Broad Spectrum Lighting and Vision",
  "url": "https://example.org/articles/broad-spectrum-lighting",
  "abstract_note": "A study of visual performance under broad spectrum illumination.",
  "creators": [
    {
      "creator_type": "author",
      "first_name": "Alex",
      "last_name": "Rivera",
      "name": null
    },
    {
      "creator_type": "author",
      "first_name": "Sam",
      "last_name": "Lee",
      "name": null
    }
  ],
  "authors": [
    "Alex Rivera",
    "Sam Lee"
  ],
  "raw_data": {
    "itemType": "journalArticle",
    "title": "Broad Spectrum Lighting and Vision",
    "url": "https://example.org/articles/broad-spectrum-lighting",
    "abstractNote": "A study of visual performance under broad spectrum illumination.",
    "creators": [
      {
        "creatorType": "author",
        "firstName": "Alex",
        "lastName": "Rivera"
      },
      {
        "creatorType": "author",
        "firstName": "Sam",
        "lastName": "Lee"
      }
    ]
  }
}
```

## License

MIT
