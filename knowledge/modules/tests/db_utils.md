---
type: Rust Module
title: db_utils
resource: tests/db_utils.rs#L1-L172
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-db-utils-init-db-insert-stamp-insert-stamp-placeholder-migrate-fix-block-metadata-sort-stampstubs-inplace-stampstub
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [make_stub](../../functions/tests/db_utils/make_stub.md)
- [test_numeric_prefix_ordering](../../functions/tests/db_utils/test_numeric_prefix_ordering.md)
- [test_insert_stamp_backfill](../../functions/tests/db_utils/test_insert_stamp_backfill.md)
- [test_insert_stamp_placeholder_allows_distinct_sg_numbers](../../functions/tests/db_utils/test_insert_stamp_placeholder_allows_distinct_sg_numbers.md)
- [test_migrate_fix_block_metadata_basic](../../functions/tests/db_utils/test_migrate_fix_block_metadata_basic.md)

# Imports

- `stamp_album_gen2::db_utils::{
    init_db, insert_stamp, insert_stamp_placeholder, migrate_fix_block_metadata,
    sort_stampstubs_inplace, StampStub,
}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)