---
type: Rust Function
title: test_first_run_without_terminal_creates_defaults
resource: tests/config_init_tests.rs#L86-L136
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_first_run_without_terminal_creates_defaults()`

# Calls

- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [success](../../../functions/tests/release_publication_shell/success.md)