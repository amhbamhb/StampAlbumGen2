---
type: Rust Function
title: invalid_title_policy_fails_with_keyed_redacted_errors
resource: tests/native_pdf_page_title.rs#L288-L327
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

`fn invalid_title_policy_fails_with_keyed_redacted_errors() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [err](../../../functions/src/external_album/err.md)