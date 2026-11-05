---
type: Rust Method
title: set_decorative_border
resource: src/native_pdf/document.rs#L137-L139
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_decorative_border(&mut self, border: AlbumPagesDecorativeBorder)`

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [numeric_content_is_unchanged_and_invalid_geometry_preserves_output](../../../../../functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output.md)