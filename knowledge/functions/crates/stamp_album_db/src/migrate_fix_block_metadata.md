---
type: Rust Function
title: migrate_fix_block_metadata
resource: crates/stamp_album_db/src/lib.rs#L729-L905
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/extract_block_metadata_local
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [extract_block_metadata_local](../../../../functions/crates/stamp_album_db/src/extract_block_metadata_local.md)