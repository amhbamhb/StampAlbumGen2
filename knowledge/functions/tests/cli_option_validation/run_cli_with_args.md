---
type: Rust Function
title: run_cli_with_args
resource: tests/cli_option_validation.rs#L8-L22
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/cli_option_validation/test_cli_help_and_version
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_no_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_source_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_year_range_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_database_path_handling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_boolean_flags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_required_combinations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_error_message_quality
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_special_characters_and_edge_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_numeric_parameter_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_cli_auto_correction_with_yes_flag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_cli_with_args(args: &[&str]) -> (i32, String, String)`

# Calls

- [args](../../../functions/tests/native_pdf_page_settings/args.md)

# Called by

- [test_cli_help_and_version](../../../functions/tests/cli_option_validation/test_cli_help_and_version.md)
- [test_cli_no_arguments](../../../functions/tests/cli_option_validation/test_cli_no_arguments.md)
- [test_cli_source_validation](../../../functions/tests/cli_option_validation/test_cli_source_validation.md)
- [test_cli_year_range_validation](../../../functions/tests/cli_option_validation/test_cli_year_range_validation.md)
- [test_cli_database_path_handling](../../../functions/tests/cli_option_validation/test_cli_database_path_handling.md)
- [test_cli_boolean_flags](../../../functions/tests/cli_option_validation/test_cli_boolean_flags.md)
- [test_cli_required_combinations](../../../functions/tests/cli_option_validation/test_cli_required_combinations.md)
- [test_cli_error_message_quality](../../../functions/tests/cli_option_validation/test_cli_error_message_quality.md)
- [test_cli_special_characters_and_edge_cases](../../../functions/tests/cli_option_validation/test_cli_special_characters_and_edge_cases.md)
- [test_cli_numeric_parameter_validation](../../../functions/tests/cli_option_validation/test_cli_numeric_parameter_validation.md)
- [test_cli_auto_correction_with_yes_flag](../../../functions/tests/cli_option_validation/test_cli_auto_correction_with_yes_flag.md)