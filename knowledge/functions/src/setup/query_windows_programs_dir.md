---
type: Rust Function
title: query_windows_programs_dir
resource: src/setup.rs#L489-L509
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_windows_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn query_windows_programs_dir() -> Option<PathBuf>`

# Calls

- [success](../../../functions/tests/release_publication_shell/success.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [ensure_windows_desktop_integration](../../../functions/src/setup/ensure_windows_desktop_integration.md)