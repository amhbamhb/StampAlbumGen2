---
type: Rust Function
title: make_test_db_multi
resource: tests/config_values_suite.rs#L13-L59
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/operations/insert_stamp_placeholder
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/config_values_suite/config_values_suite_iterates_and_changes_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_test_db_multi(db_path: &str)`

# Calls

- [insert_stamp_placeholder](../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)

# Called by

- [config_values_suite_iterates_and_changes_output](../../../functions/tests/config_values_suite/config_values_suite_iterates_and_changes_output.md)