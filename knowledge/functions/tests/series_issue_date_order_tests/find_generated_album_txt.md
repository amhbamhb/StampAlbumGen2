---
type: Rust Function
title: find_generated_album_txt
resource: tests/series_issue_date_order_tests.rs#L87-L97
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

`fn find_generated_album_txt(root: &std::path::Path, album_name: &str) -> std::path::PathBuf`

# Called by

- [test_series_sort_uses_calendar_month_then_day](../../../functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day.md)