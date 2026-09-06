---
type: Rust Module
title: src
resource: crates/stamp_album_db/src/lib.rs#L1-L1222
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
  - target: external/rusqlite-params-connection-optionalextension-result
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crates/stamp_album_db
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [insert_stamp_from_struct](../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)
- [StampInsert](../../../classes/crates/stamp_album_db/src/StampInsert.md)
- [StampStub](../../../classes/crates/stamp_album_db/src/StampStub.md)
- [get_stamps_by_series](../../../functions/crates/stamp_album_db/src/get_stamps_by_series.md)
- [get_all_series](../../../functions/crates/stamp_album_db/src/get_all_series.md)
- [leading_int_and_suffix](../../../functions/crates/stamp_album_db/src/leading_int_and_suffix.md)
- [sort_stampstubs_inplace](../../../functions/crates/stamp_album_db/src/sort_stampstubs_inplace.md)
- [is_stampworld_image_url](../../../functions/crates/stamp_album_db/src/is_stampworld_image_url.md)
- [is_colnect_page_url](../../../functions/crates/stamp_album_db/src/is_colnect_page_url.md)
- [is_colnect_image_url](../../../functions/crates/stamp_album_db/src/is_colnect_image_url.md)
- [is_real_image_candidate](../../../functions/crates/stamp_album_db/src/is_real_image_candidate.md)
- [is_placeholder_image_path](../../../functions/crates/stamp_album_db/src/is_placeholder_image_path.md)
- [cleanup_stampworld_identity_duplicates](../../../functions/crates/stamp_album_db/src/cleanup_stampworld_identity_duplicates.md)
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
- `rusqlite::{params, Connection, OptionalExtension, Result}`

# Member of

- [stamp_album_db](../../../packages/crates/stamp_album_db.md)