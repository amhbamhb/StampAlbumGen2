---
type: Rust Function
title: read_optional_i32_lossy
resource: crates/stamp_album_db/src/lib.rs#L104-L129
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/get_stamps_by_series
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_optional_i32_lossy(row: &Row<'_>, idx: usize, column_name: &str) -> Result<Option<i32>>`

# Called by

- [get_stamps_by_series](../../../../functions/crates/stamp_album_db/src/get_stamps_by_series.md)