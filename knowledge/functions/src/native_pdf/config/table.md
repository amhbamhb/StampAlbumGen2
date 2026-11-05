---
type: Rust Function
title: table
resource: src/native_pdf/config.rs#L619-L624
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

`fn table(parent: &mut Table, key: &str, prefix: &str) -> Result<Table>`

# Called by

- [parse](../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)