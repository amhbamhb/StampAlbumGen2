---
type: Rust Function
title: wide_and_tall_titles_remain_below_both_header_elements
resource: tests/native_pdf_header_number.rs#L346-L373
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
---

# Signature

`fn wide_and_tall_titles_remain_below_both_header_elements() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_page_count](../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)