---
type: Rust Function
title: is_desktop_launcher_auto_install_disabled
resource: src/setup.rs#L432-L434
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/desktop_launcher_disable_marker_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_desktop_launcher_auto_install_disabled(home: &std::path::Path) -> bool`

# Calls

- [desktop_launcher_disable_marker_path](../../../functions/src/setup/desktop_launcher_disable_marker_path.md)

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)