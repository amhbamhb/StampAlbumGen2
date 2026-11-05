---
type: Rust Function
title: boolean
resource: src/native_pdf/config.rs#L635-L642
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn boolean(table: &mut Table, key: &str, prefix: &str) -> Result<bool>`

# Called by

- [parse](../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)