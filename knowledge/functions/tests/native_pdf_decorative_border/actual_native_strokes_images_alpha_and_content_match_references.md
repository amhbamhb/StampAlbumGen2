---
type: Rust Function
title: actual_native_strokes_images_alpha_and_content_match_references
resource: tests/native_pdf_decorative_border.rs#L683-L820
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/nums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn actual_native_strokes_images_alpha_and_content_match_references() -> Result<()>`

# Calls

- [from_toml_in](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml_in.md)
- [nums](../../../functions/tests/native_pdf_decorative_border/nums.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)