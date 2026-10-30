---
type: Rust Function
title: desktop_exec_value
resource: src/setup.rs#L337-L339
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

`fn desktop_exec_value(contents: &str) -> Option<&str>`

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)