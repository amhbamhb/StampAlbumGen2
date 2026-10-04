---
type: Rust Function
title: fetch_and_filter_stamps_for_selection
resource: src/album_globals/generator_new/helpers.rs#L389-L396
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/selection/load_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn fetch_and_filter_stamps_for_selection( conn: &Connection, series: &str, start_year: Option<i32>, end_year: Option<i32>, ) -> SqlResult<Vec<Stamp>>`

# Calls

- [fetch_and_filter_stamps_impl](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)

# Called by

- [load_stamps](../../../../../functions/src/album_globals/selection/load_stamps.md)