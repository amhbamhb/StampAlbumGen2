---
type: Rust Function
title: query_windows_desktop_dir
resource: src/setup.rs#L465-L485
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_windows_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn query_windows_desktop_dir() -> Option<PathBuf>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [ensure_windows_desktop_integration](../../../functions/src/setup/ensure_windows_desktop_integration.md)