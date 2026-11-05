---
type: Rust Function
title: title_separation_and_invalid_later_page_preserve_output
resource: tests/native_pdf_header_date.rs#L672-L710
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn title_separation_and_invalid_later_page_preserve_output() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [err](../../../functions/src/external_album/err.md)