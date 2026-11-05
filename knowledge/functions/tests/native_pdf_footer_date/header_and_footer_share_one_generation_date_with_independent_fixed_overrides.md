---
type: Rust Function
title: header_and_footer_share_one_generation_date_with_independent_fixed_overrides
resource: tests/native_pdf_footer_date.rs#L333-L447
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
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn header_and_footer_share_one_generation_date_with_independent_fixed_overrides() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_context](../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)
- [write_page_geometry_proof_with_date](../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [open](../../../functions/src/external_album/open.md)