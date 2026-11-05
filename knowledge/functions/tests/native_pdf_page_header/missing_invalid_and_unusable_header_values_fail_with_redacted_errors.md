---
type: Rust Function
title: missing_invalid_and_unusable_header_values_fail_with_redacted_errors
resource: tests/native_pdf_page_header.rs#L150-L202
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

`fn missing_invalid_and_unusable_header_values_fail_with_redacted_errors() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)