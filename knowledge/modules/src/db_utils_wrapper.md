---
type: Rust Module
title: db_utils_wrapper
resource: src/db_utils_wrapper.rs#L1-L224
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-stamp-album-db-stampstub
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [init_db](../../functions/src/db_utils_wrapper/init_db.md)
- [migrate_stamps_table](../../functions/src/db_utils_wrapper/migrate_stamps_table.md)
- [migrate_fix_block_metadata](../../functions/src/db_utils_wrapper/migrate_fix_block_metadata.md)
- [insert_stamp](../../functions/src/db_utils_wrapper/insert_stamp.md)
- [insert_stamp_placeholder](../../functions/src/db_utils_wrapper/insert_stamp_placeholder.md)
- [get_stamps_by_series](../../functions/src/db_utils_wrapper/get_stamps_by_series.md)
- [get_all_series](../../functions/src/db_utils_wrapper/get_all_series.md)
- [get_stamps_by_country_series](../../functions/src/db_utils_wrapper/get_stamps_by_country_series.md)
- [format_stamps_issued](../../functions/src/db_utils_wrapper/format_stamps_issued.md)
- [sort_stampstubs_inplace](../../functions/src/db_utils_wrapper/sort_stampstubs_inplace.md)
- [update_image_dimensions](../../functions/src/db_utils_wrapper/update_image_dimensions.md)

# Imports

- `rusqlite::{Connection, Result}`
- `pub use stamp_album_db::StampStub`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)