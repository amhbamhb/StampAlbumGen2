---
type: Rust Function
title: create_test_db
resource: tests/cli_option_validation.rs#L25-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/cli_option_validation/test_cli_database_path_handling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_required_combinations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_test_db(path: &str)`

# Called by

- [test_cli_database_path_handling](../../../functions/tests/cli_option_validation/test_cli_database_path_handling.md)
- [test_cli_required_combinations](../../../functions/tests/cli_option_validation/test_cli_required_combinations.md)