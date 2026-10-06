---
type: Rust Function
title: ensure_config_and_template_files
resource: src/config.rs#L51-L437
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_default_files
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/ensure_runtime_startup_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_config_and_template_files( config_path: &str, template_path: &str, pdf_path: &str, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [skip](../../../functions/src/external_album/parser/Lexer/skip.md)
- [ensure_default_files](../../../functions/src/config/ensure_default_files.md)

# Called by

- [load_config](../../../functions/src/config/load_config.md)
- [main](../../../functions/src/main.md)
- [ensure_runtime_startup_files](../../../functions/src/review_ui/ensure_runtime_startup_files.md)
- [build_config_editor_data](../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [save_full_config_values](../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [save_album_config_diff](../../../functions/src/review_ui/config_editor/save_album_config_diff.md)
- [test_first_run_without_terminal_creates_defaults](../../../functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults.md)