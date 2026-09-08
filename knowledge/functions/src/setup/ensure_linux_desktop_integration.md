---
type: Rust Function
title: ensure_linux_desktop_integration
resource: src/setup.rs#L623-L789
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/is_cargo_build_binary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/user_home_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/is_desktop_launcher_auto_install_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/desktop_exec_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/desktop_working_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/desktop_exec_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/mark_desktop_file_trusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/bundled_icon_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/refresh_linux_desktop_caches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/install_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport>`

# Calls

- [is_cargo_build_binary](../../../functions/src/setup/is_cargo_build_binary.md)
- [user_home_dir](../../../functions/src/setup/user_home_dir.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [is_desktop_launcher_auto_install_disabled](../../../functions/src/setup/is_desktop_launcher_auto_install_disabled.md)
- [desktop_exec_path](../../../functions/src/setup/desktop_exec_path.md)
- [desktop_working_dir](../../../functions/src/setup/desktop_working_dir.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [desktop_exec_value](../../../functions/src/setup/desktop_exec_value.md)
- [mark_desktop_file_trusted](../../../functions/src/setup/mark_desktop_file_trusted.md)
- [bundled_icon_png](../../../functions/src/setup/bundled_icon_png.md)
- [refresh_linux_desktop_caches](../../../functions/src/setup/refresh_linux_desktop_caches.md)

# Called by

- [main](../../../functions/src/main.md)
- [install_linux_desktop_integration](../../../functions/src/setup/install_linux_desktop_integration.md)