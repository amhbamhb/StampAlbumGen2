---
type: Rust Function
title: get_stamps_by_series
resource: crates/stamp_album_db/src/lib.rs#L423-L472
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_stamps_by_series( conn: &Connection, series: &str, _year: &str, ) -> Result<Vec<StampStub>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)