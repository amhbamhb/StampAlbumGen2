---
type: Rust Function
title: defaults_missing_invalid_values_and_snapshot_are_explicit
resource: tests/native_pdf_header_number.rs#L86-L229
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_page_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn defaults_missing_invalid_values_and_snapshot_are_explicit() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_page_count](../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)
- [err](../../../functions/src/external_album/err.md)