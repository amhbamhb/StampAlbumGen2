---
type: Rust Module
title: ingest
resource: src/colnect/ingest.rs#L1-L330
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-colnect-parser-colnectstamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection-result
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ingest_colnect_data](../../../functions/src/colnect/ingest/ingest_colnect_data.md)
- [ingest_colnect_data_with_default_catalog](../../../functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog.md)
- [ingest_colnect_data_with_mode](../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)
- [link_colnect_data](../../../functions/src/colnect/ingest/link_colnect_data.md)
- [extract_colnect_code](../../../functions/src/colnect/ingest/extract_colnect_code.md)

# Imports

- `crate::colnect::parser::ColnectStamp`
- `log::info`
- `regex::Regex`
- `rusqlite::{params, Connection, Result}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)