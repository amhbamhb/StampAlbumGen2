---
type: Rust Module
title: legacy
resource: src/wikipedia/legacy.rs#L1-L69
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-stampworld-fetch-fetcher-reqwestfetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-wikipedia-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [fetch_and_cache_wikipedia_content](../../../functions/src/wikipedia/legacy/fetch_and_cache_wikipedia_content.md)
- [get_cached_wikipedia_content](../../../functions/src/wikipedia/legacy/get_cached_wikipedia_content.md)
- [fetch_and_save_summary](../../../functions/src/wikipedia/legacy/fetch_and_save_summary.md)
- [fetch_and_save_summary_with_fetcher](../../../functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher.md)

# Imports

- `crate::stampworld::fetch::{Fetcher, ReqwestFetcher}`
- `crate::wikipedia::database`
- `std::fs`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)