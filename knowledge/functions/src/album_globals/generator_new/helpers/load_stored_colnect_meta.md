---
type: Rust Function
title: load_stored_colnect_meta
resource: src/album_globals/generator_new/helpers.rs#L288-L365
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_stored_colnect_meta( conn: &Connection, stamp: &crate::db_utils::StampStub, ) -> SqlResult<Option<StoredColnectMeta>>`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [fetch_and_filter_stamps_impl](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)