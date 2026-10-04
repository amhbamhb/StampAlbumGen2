---
type: Rust Module
title: unit_tests
resource: crates/stamp_album_db/tests/unit_tests.rs#L1-L231
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-db-get-all-series-get-stamps-by-series-insert-stamp-from-struct-migrate-fix-block-metadata-sort-stampstubs-inplace-stampinsert-stampstub
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crates/stamp_album_db
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [make_stub](../../../../functions/crates/stamp_album_db/tests/unit_tests/make_stub.md)
- [test_sort_stampstubs_inplace_basic](../../../../functions/crates/stamp_album_db/tests/unit_tests/test_sort_stampstubs_inplace_basic.md)
- [test_get_all_series_and_get_stamps_by_series](../../../../functions/crates/stamp_album_db/tests/unit_tests/test_get_all_series_and_get_stamps_by_series.md)
- [test_insert_stamp_and_backfill](../../../../functions/crates/stamp_album_db/tests/unit_tests/test_insert_stamp_and_backfill.md)
- [test_migrate_fix_block_metadata_local](../../../../functions/crates/stamp_album_db/tests/unit_tests/test_migrate_fix_block_metadata_local.md)

# Imports

- `rusqlite::Connection`
- `stamp_album_db::{
    get_all_series, get_stamps_by_series, insert_stamp_from_struct, migrate_fix_block_metadata,
    sort_stampstubs_inplace, StampInsert, StampStub,
}`

# Member of

- [stamp_album_db](../../../../packages/crates/stamp_album_db.md)