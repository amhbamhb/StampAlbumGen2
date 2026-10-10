---
type: Rust Module
title: cli_option_validation
resource: tests/cli_option_validation.rs#L1-L513
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [run_cli_with_args](../../functions/tests/cli_option_validation/run_cli_with_args.md)
- [create_test_db](../../functions/tests/cli_option_validation/create_test_db.md)
- [test_cli_help_and_version](../../functions/tests/cli_option_validation/test_cli_help_and_version.md)
- [test_cli_no_arguments](../../functions/tests/cli_option_validation/test_cli_no_arguments.md)
- [test_cli_source_validation](../../functions/tests/cli_option_validation/test_cli_source_validation.md)
- [test_cli_year_range_validation](../../functions/tests/cli_option_validation/test_cli_year_range_validation.md)
- [test_cli_database_path_handling](../../functions/tests/cli_option_validation/test_cli_database_path_handling.md)
- [test_cli_boolean_flags](../../functions/tests/cli_option_validation/test_cli_boolean_flags.md)
- [test_cli_required_combinations](../../functions/tests/cli_option_validation/test_cli_required_combinations.md)
- [test_cli_error_message_quality](../../functions/tests/cli_option_validation/test_cli_error_message_quality.md)
- [test_cli_special_characters_and_edge_cases](../../functions/tests/cli_option_validation/test_cli_special_characters_and_edge_cases.md)
- [test_cli_numeric_parameter_validation](../../functions/tests/cli_option_validation/test_cli_numeric_parameter_validation.md)
- [test_cli_auto_correction_with_yes_flag](../../functions/tests/cli_option_validation/test_cli_auto_correction_with_yes_flag.md)
- [test_ensure_binary_built](../../functions/tests/cli_option_validation/test_ensure_binary_built.md)

# Imports

- `std::process::Command`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)