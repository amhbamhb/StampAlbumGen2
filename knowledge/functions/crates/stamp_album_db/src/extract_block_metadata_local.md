---
type: Rust Function
title: extract_block_metadata_local
resource: crates/stamp_album_db/src/lib.rs#L683-L725
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/crates/stamp_album_db/src/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_block_metadata_local(details: &[String]) -> (String, String, String, String)`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [migrate_fix_block_metadata](../../../../functions/crates/stamp_album_db/src/migrate_fix_block_metadata.md)