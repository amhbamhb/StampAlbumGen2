---
type: Rust Function
title: set_desktop_launcher_auto_install_disabled
resource: src/setup.rs#L436-L451
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/desktop_launcher_disable_marker_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/app_state_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/uninstall_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/install_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_desktop_launcher_auto_install_disabled( home: &std::path::Path, disabled: bool, ) -> std::io::Result<()>`

# Calls

- [desktop_launcher_disable_marker_path](../../../functions/src/setup/desktop_launcher_disable_marker_path.md)
- [app_state_dir](../../../functions/src/setup/app_state_dir.md)

# Called by

- [uninstall_linux_desktop_integration](../../../functions/src/setup/uninstall_linux_desktop_integration.md)
- [install_linux_desktop_integration](../../../functions/src/setup/install_linux_desktop_integration.md)