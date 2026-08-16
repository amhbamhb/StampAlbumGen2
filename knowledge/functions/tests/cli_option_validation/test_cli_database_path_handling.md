---
type: Rust Function
title: test_cli_database_path_handling
resource: tests/cli_option_validation.rs#L222-L266
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/cli_option_validation/create_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/run_cli_with_args
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_cli_database_path_handling()`

# Calls

- [create_test_db](../../../functions/tests/cli_option_validation/create_test_db.md)
- [run_cli_with_args](../../../functions/tests/cli_option_validation/run_cli_with_args.md)