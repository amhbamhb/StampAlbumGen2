---
type: Rust Function
title: install_linux_desktop_integration
resource: src/setup.rs#L901-L914
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/user_home_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/set_desktop_launcher_auto_install_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn install_linux_desktop_integration() -> std::io::Result<LinuxDesktopIntegrationReport>`

# Calls

- [user_home_dir](../../../functions/src/setup/user_home_dir.md)
- [set_desktop_launcher_auto_install_disabled](../../../functions/src/setup/set_desktop_launcher_auto_install_disabled.md)
- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)

# Called by

- [main](../../../functions/src/main.md)