---
type: Rust Function
title: migrate_stamps_table
resource: crates/stamp_album_db/src/lib.rs#L638-L660
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn migrate_stamps_table(conn: &Connection) -> Result<()>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)