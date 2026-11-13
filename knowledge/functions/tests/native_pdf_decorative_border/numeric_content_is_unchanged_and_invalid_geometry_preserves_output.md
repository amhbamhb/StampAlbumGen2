---
type: Rust Function
title: numeric_content_is_unchanged_and_invalid_geometry_preserves_output
resource: tests/native_pdf_decorative_border.rs#L463-L513
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_decorative_border/loaded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_decorative_border
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn numeric_content_is_unchanged_and_invalid_geometry_preserves_output() -> Result<()>`

# Calls

- [loaded](../../../functions/tests/native_pdf_decorative_border/loaded.md)
- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_decorative_border](../../../functions/src/native_pdf/document/PageSettings/set_decorative_border.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)