---
type: Rust Function
title: footer_fonts_fit_spill_and_no_partial_output
resource: tests/native_pdf_page_footer.rs#L247-L308
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn footer_fonts_fit_spill_and_no_partial_output() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)