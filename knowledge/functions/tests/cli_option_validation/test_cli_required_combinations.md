---
type: Rust Function
title: test_cli_required_combinations
resource: tests/cli_option_validation.rs#L305-L330
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/cli_option_validation/run_cli_with_args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/create_test_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_cli_required_combinations()`

# Calls

- [run_cli_with_args](../../../functions/tests/cli_option_validation/run_cli_with_args.md)
- [create_test_db](../../../functions/tests/cli_option_validation/create_test_db.md)