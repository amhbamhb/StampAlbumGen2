---
type: Rust Function
title: load_existing_stamp
resource: crates/stamp_album_db/src/lib.rs#L320-L353
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_existing_stamp( conn: &Connection, predicate: &str, bind: &[&dyn rusqlite::ToSql], ) -> Result<Option<ExistingStamp>>`

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)