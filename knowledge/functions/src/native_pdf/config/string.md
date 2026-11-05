---
type: Rust Function
title: string
resource: src/native_pdf/config.rs#L626-L633
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

`fn string(table: &mut Table, key: &str, prefix: &str) -> Result<String>`

# Called by

- [parse](../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)