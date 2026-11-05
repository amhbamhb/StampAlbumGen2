---
type: Rust Function
title: actual_native_paths_match_all_repeated_reference_cases
resource: tests/native_pdf_crop_marks.rs#L522-L578
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_crop_marks/cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_crop_marks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_border_colour
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn actual_native_paths_match_all_repeated_reference_cases() -> Result<()>`

# Calls

- [cases](../../../functions/tests/native_pdf_crop_marks/cases.md)
- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_crop_marks](../../../functions/src/native_pdf/document/PageSettings/set_crop_marks.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [set_border_colour](../../../functions/src/native_pdf/document/PageSettings/set_border_colour.md)
- [open](../../../functions/src/external_album/open.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)