---
type: Rust Function
title: user_home_dir
resource: src/setup.rs#L393-L397
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/uninstall_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/install_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn user_home_dir() -> std::io::Result<std::path::PathBuf>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)
- [uninstall_linux_desktop_integration](../../../functions/src/setup/uninstall_linux_desktop_integration.md)
- [install_linux_desktop_integration](../../../functions/src/setup/install_linux_desktop_integration.md)