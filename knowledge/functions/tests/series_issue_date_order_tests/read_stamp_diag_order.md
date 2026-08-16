---
type: Rust Function
title: read_stamp_diag_order
resource: tests/series_issue_date_order_tests.rs#L71-L85
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_stamp_diag_order(output_txt: &std::path::Path) -> Vec<String>`

# Called by

- [test_series_sort_uses_calendar_month_then_day](../../../functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day.md)