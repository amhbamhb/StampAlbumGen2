---
type: Rust Function
title: is_cargo_build_binary
resource: src/setup.rs#L183-L193
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_cargo_build_binary() -> bool`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)