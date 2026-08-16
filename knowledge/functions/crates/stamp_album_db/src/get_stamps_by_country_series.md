---
type: Rust Function
title: get_stamps_by_country_series
resource: crates/stamp_album_db/src/lib.rs#L1032-L1061
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_stamps_by_country_series( conn: &Connection, country: &str, series: &str, ) -> Result<Vec<StampCountrySeriesRow>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)