---
type: Rust Module
title: src
resource: crates/stamp_album_db/src/lib.rs#L1-L1061
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-types-valueref-connection-result-row
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-format-stamps-issued-get-stamps-by-series
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crates/stamp_album_db
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [StampStub](../../../classes/crates/stamp_album_db/src/StampStub.md)
- [get_all_series](../../../functions/crates/stamp_album_db/src/get_all_series.md)
- [leading_int](../../../functions/crates/stamp_album_db/src/leading_int.md)
- [suffix_key](../../../functions/crates/stamp_album_db/src/suffix_key.md)
- [read_optional_i32_lossy](../../../functions/crates/stamp_album_db/src/read_optional_i32_lossy.md)
- [format_stamps_issued](../../../functions/crates/stamp_album_db/src/format_stamps_issued.md)
- [test_format_integer_only](../../../functions/crates/stamp_album_db/src/test_format_integer_only.md)
- [test_format_with_decimal](../../../functions/crates/stamp_album_db/src/test_format_with_decimal.md)
- [test_insert_placeholder](../../../functions/crates/stamp_album_db/src/test_insert_placeholder.md)
- [test_get_stamps_by_series_tolerates_text_image_dimensions](../../../functions/crates/stamp_album_db/src/test_get_stamps_by_series_tolerates_text_image_dimensions.md)
- [insert_stamp_placeholder](../../../functions/crates/stamp_album_db/src/insert_stamp_placeholder.md)
- [sort_stampstubs_inplace](../../../functions/crates/stamp_album_db/src/sort_stampstubs_inplace.md)
- [get_stamps_by_series](../../../functions/crates/stamp_album_db/src/get_stamps_by_series.md)
- [init_db](../../../functions/crates/stamp_album_db/src/init_db.md)
- [migrate_stamps_table](../../../functions/crates/stamp_album_db/src/migrate_stamps_table.md)
- [migrate_fix_block_metadata](../../../functions/crates/stamp_album_db/src/migrate_fix_block_metadata.md)
- [extract_block_metadata_local](../../../functions/crates/stamp_album_db/src/extract_block_metadata_local.md)
- [normalize_currency_token_inner](../../../functions/crates/stamp_album_db/src/normalize_currency_token_inner.md)
- [insert_stamp](../../../functions/crates/stamp_album_db/src/insert_stamp.md)
- [get_stamps_by_country_series](../../../functions/crates/stamp_album_db/src/get_stamps_by_country_series.md)

# Imports

- `log::{debug, error}`
- `regex::Regex`
- `rusqlite::{params, types::ValueRef, Connection, Result, Row}`
- `super::{format_stamps_issued, get_stamps_by_series}`

# Member of

- [stamp_album_db](../../../packages/crates/stamp_album_db.md)