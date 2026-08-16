---
type: Rust Module
title: operations
resource: src/db_utils/operations.rs#L1-L684
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
  - target: external/crate-db-utils-stampstub
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [cleanup_stampworld_identity_duplicates](../../../functions/src/db_utils/operations/cleanup_stampworld_identity_duplicates.md)
- [is_placeholder_image_path](../../../functions/src/db_utils/operations/is_placeholder_image_path.md)
- [is_real_image_candidate](../../../functions/src/db_utils/operations/is_real_image_candidate.md)
- [is_image_url](../../../functions/src/db_utils/operations/is_image_url.md)
- [is_stampworld_image_url](../../../functions/src/db_utils/operations/is_stampworld_image_url.md)
- [is_colnect_page_url](../../../functions/src/db_utils/operations/is_colnect_page_url.md)
- [is_colnect_image_url](../../../functions/src/db_utils/operations/is_colnect_image_url.md)
- [insert_stamp](../../../functions/src/db_utils/operations/insert_stamp.md)
- [insert_stamp_placeholder](../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)
- [get_stamps_by_country_series](../../../functions/src/db_utils/operations/get_stamps_by_country_series.md)
- [get_stamps_by_series](../../../functions/src/db_utils/operations/get_stamps_by_series.md)
- [get_all_series](../../../functions/src/db_utils/operations/get_all_series.md)

# Imports

- `log::{debug, error}`
- `regex::Regex`
- `rusqlite::{params, Connection, OptionalExtension, Result}`
- `crate::db_utils::StampStub`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)