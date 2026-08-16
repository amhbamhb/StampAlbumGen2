---
type: Rust Function
title: desktop_working_dir
resource: src/setup.rs#L210-L222
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn desktop_working_dir() -> std::io::Result<String>`

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)