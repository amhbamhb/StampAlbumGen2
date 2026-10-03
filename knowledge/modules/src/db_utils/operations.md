---
type: Rust Module
title: operations
resource: src/db_utils/operations.rs#L1-L365
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-utils-stampstub
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-db-stampinsert-as-dbstampinsert
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [insert_stamp](../../../functions/src/db_utils/operations/insert_stamp.md)
- [StampInsert](../../../classes/src/db_utils/operations/StampInsert.md)
- [insert_stamp_from_struct](../../../functions/src/db_utils/operations/insert_stamp_from_struct.md)
- [insert_stamp_placeholder](../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)
- [get_stamps_by_country_series](../../../functions/src/db_utils/operations/get_stamps_by_country_series.md)
- [get_stamps_by_series](../../../functions/src/db_utils/operations/get_stamps_by_series.md)
- [get_all_series](../../../functions/src/db_utils/operations/get_all_series.md)

# Imports

- `log::{debug, error}`
- `rusqlite::{params, Connection, Result}`
- `crate::db_utils::StampStub`
- `stamp_album_db::StampInsert as DbStampInsert`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)