---
type: Rust Module
title: stampworld_debug
resource: src/bin/stampworld_debug.rs#L1-L139
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-types-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [LocalFetcher](../../../classes/src/bin/stampworld_debug/LocalFetcher.md)
- [new_from_path](../../../functions/src/bin/stampworld_debug/LocalFetcher/new_from_path.md)
- [fetch_text](../../../functions/src/bin/stampworld_debug/LocalFetcher/stamp-album-gen2-stampworld-fetch-fetcher/fetch_text.md)
- [fetch_bytes](../../../functions/src/bin/stampworld_debug/LocalFetcher/stamp-album-gen2-stampworld-fetch-fetcher/fetch_bytes.md)
- [month_number](../../../functions/src/bin/stampworld_debug/month_number.md)
- [main](../../../functions/src/bin/stampworld_debug/main.md)

# Imports

- `std::env`
- `std::fs`
- `std::path::Path`
- `reqwest::StatusCode`
- `stamp_album_gen2::types::ImageStatus`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)