---
type: Rust Function
title: defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit
resource: tests/native_pdf_header_date.rs#L396-L460
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_context](../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)