---
type: Rust Module
title: data_collector
resource: src/stampworld/data_collector.rs#L1-L1631
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-types-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug-error-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/once-cell-sync-lazy
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params
    resolved_by: tree-sitter
    confidence: exact
  - target: external/scraper-html-selector
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-stampworld-fetch-fetcher-reqwestfetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-stampworld-parser-compress-context-dedupe-details-extract-block-metadata-normalize-stamps-issued
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CollectStampworldParams](../../../classes/src/stampworld/data_collector/CollectStampworldParams.md)
- [collect_stampworld_data_async](../../../functions/src/stampworld/data_collector/collect_stampworld_data_async.md)
- [collect_stampworld_data_with_fetcher](../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [normalize_currency_token](../../../functions/src/stampworld/data_collector/normalize_currency_token.md)
- [collect_stampworld_data](../../../functions/src/stampworld/data_collector/collect_stampworld_data.md)

# Imports

- `crate::types::ImageStatus`
- `log::{debug, error, info}`
- `once_cell::sync::Lazy`
- `regex::Regex`
- `rusqlite::params`
- `scraper::{Html, Selector}`
- `std::collections::HashSet`
- `std::path::PathBuf`
- `crate::stampworld::fetch::{Fetcher, ReqwestFetcher}`
- `crate::stampworld::parser::{
    compress_context, dedupe_details, extract_block_metadata, normalize_stamps_issued,
}`
- `std::collections::HashMap`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)