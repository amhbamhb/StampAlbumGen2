---
type: Rust Module
title: flag_pipeline
resource: tests/flag_pipeline.rs#L1-L157
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-header-headermap-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-stampworld-fetch-fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [RestoreEnvironment](../../classes/tests/flag_pipeline/RestoreEnvironment.md)
- [drop](../../functions/tests/flag_pipeline/RestoreEnvironment/drop/drop.md)
- [FakeFetcher](../../classes/tests/flag_pipeline/FakeFetcher.md)
- [fetch_text](../../functions/tests/flag_pipeline/FakeFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../functions/tests/flag_pipeline/FakeFetcher/fetcher/fetch_bytes.md)
- [public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag](../../functions/tests/flag_pipeline/public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag.md)

# Imports

- `async_trait::async_trait`
- `reqwest::{header::HeaderMap, StatusCode}`
- `stamp_album_gen2::stampworld::fetch::Fetcher`
- `std::{path::PathBuf, sync::Mutex}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)