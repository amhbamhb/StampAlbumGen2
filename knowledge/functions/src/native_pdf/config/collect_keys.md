---
type: Rust Function
title: collect_keys
resource: src/native_pdf/config.rs#L663-L676
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

`fn collect_keys(table: &Table, prefix: &str, keys: &mut Vec<String>)`

# Called by

- [parse](../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)