---
type: Rust Function
title: fetch_and_filter_stamps
resource: src/album_globals/generator_new/helpers.rs#L400-L407
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/fetch_and_filter_stamps/test_fetch_and_filter_stamps_basic
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fetch_and_filter_stamps( conn: &Connection, series: &str, start_year: Option<i32>, end_year: Option<i32>, ) -> SqlResult<Vec<Stamp>>`

# Calls

- [fetch_and_filter_stamps_impl](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [test_fetch_and_filter_stamps_basic](../../../../../functions/tests/fetch_and_filter_stamps/test_fetch_and_filter_stamps_basic.md)