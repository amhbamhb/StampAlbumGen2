---
type: Rust Function
title: try_spawn_browser_command
resource: src/review_ui.rs#L1533-L1541
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/try_spawn_browser_command_any
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_spawn_browser_command(program: &str, args: &[&str]) -> bool`

# Calls

- [args](../../../functions/tests/native_pdf_page_settings/args.md)

# Called by

- [try_spawn_browser_command_any](../../../functions/src/review_ui/try_spawn_browser_command_any.md)