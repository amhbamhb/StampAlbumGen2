---
type: Rust Function
title: late_footer_date_failure_preserves_existing_output_and_title_clearance
resource: tests/native_pdf_footer_date.rs#L738-L782
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

`fn late_footer_date_failure_preserves_existing_output_and_title_clearance() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [err](../../../functions/src/external_album/err.md)