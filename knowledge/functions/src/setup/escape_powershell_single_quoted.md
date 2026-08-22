---
type: Rust Function
title: escape_powershell_single_quoted
resource: src/setup.rs#L461-L463
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/create_windows_shortcut
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn escape_powershell_single_quoted(text: &str) -> String`

# Called by

- [create_windows_shortcut](../../../functions/src/setup/create_windows_shortcut.md)