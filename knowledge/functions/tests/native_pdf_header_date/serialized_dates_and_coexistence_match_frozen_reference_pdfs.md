---
type: Rust Function
title: serialized_dates_and_coexistence_match_frozen_reference_pdfs
resource: tests/native_pdf_header_date.rs#L23-L269
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
  - target: functions/src/native_pdf/date/DateFormat/format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn serialized_dates_and_coexistence_match_frozen_reference_pdfs() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [resolve_with_page_count](../../../functions/src/native_pdf/document/PageSettings/resolve_with_page_count.md)
- [open](../../../functions/src/external_album/open.md)
- [format](../../../functions/src/native_pdf/date/DateFormat/format.md)