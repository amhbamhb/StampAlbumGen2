---
type: Rust Function
title: desktop_launcher_disable_marker_path
resource: src/setup.rs#L428-L430
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/app_state_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/is_desktop_launcher_auto_install_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/set_desktop_launcher_auto_install_disabled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn desktop_launcher_disable_marker_path(home: &std::path::Path) -> std::path::PathBuf`

# Calls

- [app_state_dir](../../../functions/src/setup/app_state_dir.md)

# Called by

- [is_desktop_launcher_auto_install_disabled](../../../functions/src/setup/is_desktop_launcher_auto_install_disabled.md)
- [set_desktop_launcher_auto_install_disabled](../../../functions/src/setup/set_desktop_launcher_auto_install_disabled.md)