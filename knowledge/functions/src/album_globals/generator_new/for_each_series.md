---
type: Rust Function
title: for_each_series
resource: src/album_globals/generator_new/mod.rs#L73-L82
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/for_each_series/test_for_each_series_basic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/series_issue_date_order_tests/test_series_order_uses_earliest_issue_date
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn for_each_series<F>(conn: &Connection, _order: Option<&str>, mut cb: F) -> SqlResult<()> where F: FnMut(usize, &str),`

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [test_for_each_series_basic](../../../../functions/tests/for_each_series/test_for_each_series_basic.md)
- [test_series_order_uses_earliest_issue_date](../../../../functions/tests/series_issue_date_order_tests/test_series_order_uses_earliest_issue_date.md)