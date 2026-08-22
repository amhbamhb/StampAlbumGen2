---
type: Rust Function
title: get_stamps_by_series
resource: crates/stamp_album_db/src/lib.rs#L358-L447
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/crates/stamp_album_db/src/read_optional_i32_lossy
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_stamps_by_series(conn: &Connection, series: &str, year: &str) -> Result<Vec<StampStub>>`

# Calls

- [read_optional_i32_lossy](../../../../functions/crates/stamp_album_db/src/read_optional_i32_lossy.md)