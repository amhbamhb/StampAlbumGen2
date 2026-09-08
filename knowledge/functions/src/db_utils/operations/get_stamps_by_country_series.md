---
type: Rust Function
title: get_stamps_by_country_series
resource: src/db_utils/operations.rs#L183-L227
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_stamps_by_country_series( conn: &Connection, country: &str, series: &str, ) -> Result< Vec<( i32, String, String, i32, String, i32, String, String, String, String, String, String, )>, >`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)