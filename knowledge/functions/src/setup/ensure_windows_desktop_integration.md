---
type: Rust Function
title: ensure_windows_desktop_integration
resource: src/setup.rs#L547-L621
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/query_windows_desktop_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/query_windows_programs_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/bundled_windows_icon_ico
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/create_windows_shortcut
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_windows_desktop_integration() -> std::io::Result<WindowsDesktopIntegrationReport>`

# Calls

- [query_windows_desktop_dir](../../../functions/src/setup/query_windows_desktop_dir.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [query_windows_programs_dir](../../../functions/src/setup/query_windows_programs_dir.md)
- [bundled_windows_icon_ico](../../../functions/src/setup/bundled_windows_icon_ico.md)
- [create_windows_shortcut](../../../functions/src/setup/create_windows_shortcut.md)

# Called by

- [main](../../../functions/src/main.md)