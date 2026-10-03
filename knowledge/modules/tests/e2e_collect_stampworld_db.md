---
type: Rust Module
title: e2e_collect_stampworld_db
resource: tests/e2e_collect_stampworld_db.rs#L1-L173
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-header-headermap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-stampworld-fetch-fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FakeFetcher](../../classes/tests/e2e_collect_stampworld_db/FakeFetcher.md)
- [new](../../functions/tests/e2e_collect_stampworld_db/FakeFetcher/new.md)
- [fetch_text](../../functions/tests/e2e_collect_stampworld_db/FakeFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../functions/tests/e2e_collect_stampworld_db/FakeFetcher/fetcher/fetch_bytes.md)
- [e2e_collect_stampworld_inserts_placeholders_to_db](../../functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db.md)

# Imports

- `async_trait::async_trait`
- `reqwest::header::HeaderMap`
- `reqwest::StatusCode`
- `stamp_album_gen2::stampworld::fetch::Fetcher`
- `std::env`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)