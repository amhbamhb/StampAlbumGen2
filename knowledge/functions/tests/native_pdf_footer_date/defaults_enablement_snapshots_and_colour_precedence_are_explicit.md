---
type: Rust Function
title: defaults_enablement_snapshots_and_colour_precedence_are_explicit
resource: tests/native_pdf_footer_date.rs#L450-L535
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

`fn defaults_enablement_snapshots_and_colour_precedence_are_explicit() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_context](../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)