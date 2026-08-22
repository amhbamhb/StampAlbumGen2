---
type: Rust Function
title: command_exists
resource: src/setup.rs#L339-L346
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/refresh_linux_desktop_caches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/mark_desktop_file_trusted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn command_exists(name: &str) -> bool`

# Called by

- [refresh_linux_desktop_caches](../../../functions/src/setup/refresh_linux_desktop_caches.md)
- [mark_desktop_file_trusted](../../../functions/src/setup/mark_desktop_file_trusted.md)