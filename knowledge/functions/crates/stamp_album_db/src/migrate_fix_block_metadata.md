---
type: Rust Function
title: migrate_fix_block_metadata
resource: crates/stamp_album_db/src/lib.rs#L661-L837
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/extract_block_metadata_local
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [extract_block_metadata_local](../../../../functions/crates/stamp_album_db/src/extract_block_metadata_local.md)