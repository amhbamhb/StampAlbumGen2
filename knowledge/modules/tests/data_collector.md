---
type: Rust Module
title: data_collector
resource: tests/data_collector.rs#L1-L71
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-stampworld-data-collector-day-month-re
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-stampworld-parser-dedupe-details-extract-block-metadata-normalize-stamps-issued
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [test_normalize_stamps_issued_basic](../../functions/tests/data_collector/test_normalize_stamps_issued_basic.md)
- [test_dedupe_details_preserves_order](../../functions/tests/data_collector/test_dedupe_details_preserves_order.md)
- [test_extract_block_metadata](../../functions/tests/data_collector/test_extract_block_metadata.md)
- [test_extract_block_metadata_multiline_block](../../functions/tests/data_collector/test_extract_block_metadata_multiline_block.md)
- [test_day_month_regex_unicode](../../functions/tests/data_collector/test_day_month_regex_unicode.md)

# Imports

- `stamp_album_gen2::stampworld::data_collector::DAY_MONTH_RE`
- `stamp_album_gen2::stampworld::parser::{
    dedupe_details, extract_block_metadata, normalize_stamps_issued,
}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)