---
type: Rust Function
title: get_all_series
resource: crates/stamp_album_db/src/lib.rs#L475-L485
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_all_series(conn: &Connection) -> Result<Vec<String>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)