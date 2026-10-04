---
type: Rust Function
title: config_values_suite_iterates_and_changes_output
resource: tests/config_values_suite.rs#L62-L263
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/config_values_suite/make_test_db_multi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn config_values_suite_iterates_and_changes_output()`

# Calls

- [make_test_db_multi](../../../functions/tests/config_values_suite/make_test_db_multi.md)
- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)