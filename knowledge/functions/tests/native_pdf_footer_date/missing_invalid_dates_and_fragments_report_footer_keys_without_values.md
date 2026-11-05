---
type: Rust Function
title: missing_invalid_dates_and_fragments_report_footer_keys_without_values
resource: tests/native_pdf_footer_date.rs#L538-L635
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
---

# Signature

`fn missing_invalid_dates_and_fragments_report_footer_keys_without_values() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)