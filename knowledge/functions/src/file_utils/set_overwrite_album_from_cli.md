---
type: Rust Function
title: set_overwrite_album_from_cli
resource: src/file_utils.rs#L91-L94
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/test_config_with_actual_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values/test_stamp_heading_size_changes_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_label/test_stamp_heading_label_changes_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_simple/config_values_simple_no_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_suite/config_values_suite_iterates_and_changes_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/tmp_file_flushed/test_temp_file_flush_and_output_non_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_overwrite_album_from_cli(flag: bool)`

# Called by

- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [test_config_with_actual_generation](../../../functions/tests/config_parameter_validation/test_config_with_actual_generation.md)
- [test_stamp_heading_size_changes_output](../../../functions/tests/config_values/test_stamp_heading_size_changes_output.md)
- [test_stamp_heading_label_changes_output](../../../functions/tests/config_values_label/test_stamp_heading_label_changes_output.md)
- [config_values_simple_no_placeholder](../../../functions/tests/config_values_simple/config_values_simple_no_placeholder.md)
- [config_values_suite_iterates_and_changes_output](../../../functions/tests/config_values_suite/config_values_suite_iterates_and_changes_output.md)
- [test_series_sort_uses_calendar_month_then_day](../../../functions/tests/series_issue_date_order_tests/test_series_sort_uses_calendar_month_then_day.md)
- [test_temp_file_flush_and_output_non_empty](../../../functions/tests/tmp_file_flushed/test_temp_file_flush_and_output_non_empty.md)