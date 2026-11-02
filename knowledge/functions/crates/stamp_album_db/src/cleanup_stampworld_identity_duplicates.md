---
type: Rust Function
title: cleanup_stampworld_identity_duplicates
resource: crates/stamp_album_db/src/lib.rs#L583-L591
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

`fn cleanup_stampworld_identity_duplicates( _conn: &Connection, _existing_rowid: i64, _country: &str, _url: &str, ) -> Result<()>`

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)