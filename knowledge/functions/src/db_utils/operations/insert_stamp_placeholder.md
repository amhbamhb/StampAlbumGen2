---
type: Rust Function
title: insert_stamp_placeholder
resource: src/db_utils/operations.rs#L511-L552
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_stamp_placeholder( conn: &Connection, country: &str, series: &str, year: i32, sg_number: &str, ) -> Result<bool>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)