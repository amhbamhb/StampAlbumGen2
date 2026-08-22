---
type: Rust Function
title: test_series_sort_uses_calendar_month_then_day
resource: tests/series_issue_date_order_tests.rs#L101-L131
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/series_issue_date_order_tests/find_generated_album_txt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/series_issue_date_order_tests/read_stamp_diag_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_series_sort_uses_calendar_month_then_day()`

# Calls

- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [find_generated_album_txt](../../../functions/tests/series_issue_date_order_tests/find_generated_album_txt.md)
- [read_stamp_diag_order](../../../functions/tests/series_issue_date_order_tests/read_stamp_diag_order.md)