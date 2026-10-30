---
type: Rust Function
title: app_state_dir
resource: src/setup.rs#L426-L428
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/desktop_launcher_disable_marker_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/set_desktop_launcher_auto_install_disabled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn app_state_dir(home: &std::path::Path) -> std::path::PathBuf`

# Called by

- [desktop_launcher_disable_marker_path](../../../functions/src/setup/desktop_launcher_disable_marker_path.md)
- [set_desktop_launcher_auto_install_disabled](../../../functions/src/setup/set_desktop_launcher_auto_install_disabled.md)