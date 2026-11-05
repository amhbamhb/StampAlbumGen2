---
type: Rust Function
title: serialized_numbers_tokens_and_footer_coexistence_match_references
resource: tests/native_pdf_footer_number.rs#L452-L637
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
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn serialized_numbers_tokens_and_footer_coexistence_match_references() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_page_count](../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)
- [open](../../../functions/src/external_album/open.md)