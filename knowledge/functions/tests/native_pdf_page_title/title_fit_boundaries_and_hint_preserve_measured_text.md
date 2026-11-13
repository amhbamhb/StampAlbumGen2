---
type: Rust Function
title: title_fit_boundaries_and_hint_preserve_measured_text
resource: tests/native_pdf_page_title.rs#L106-L195
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_title
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn title_fit_boundaries_and_hint_preserve_measured_text() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_title](../../../functions/src/native_pdf/document/PageSettings/set_title.md)