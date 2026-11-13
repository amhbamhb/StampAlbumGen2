---
type: Rust Function
title: invalid_layout_creates_no_pdf_and_preserves_existing_output
resource: tests/native_pdf_crop_marks.rs#L314-L339
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_crop_marks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn invalid_layout_creates_no_pdf_and_preserves_existing_output() -> Result<()>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_crop_marks](../../../functions/src/native_pdf/document/PageSettings/set_crop_marks.md)