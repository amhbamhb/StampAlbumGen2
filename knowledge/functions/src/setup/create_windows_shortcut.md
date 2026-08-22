---
type: Rust Function
title: create_windows_shortcut
resource: src/setup.rs#L509-L545
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/escape_powershell_single_quoted
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_windows_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_windows_shortcut( shortcut_path: &Path, target: &Path, working_dir: &Path, icon_path: &Path, ) -> std::io::Result<()>`

# Calls

- [escape_powershell_single_quoted](../../../functions/src/setup/escape_powershell_single_quoted.md)

# Called by

- [ensure_windows_desktop_integration](../../../functions/src/setup/ensure_windows_desktop_integration.md)