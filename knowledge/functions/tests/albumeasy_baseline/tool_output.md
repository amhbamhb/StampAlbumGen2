---
type: Rust Function
title: tool_output
resource: tests/albumeasy_baseline.rs#L187-L199
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/albumeasy_baseline/inspect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_output(program: &str, args: &[&OsStr]) -> Result<String>`

# Calls

- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [success](../../../functions/tests/release_publication_shell/success.md)

# Called by

- [inspect](../../../functions/tests/albumeasy_baseline/inspect.md)