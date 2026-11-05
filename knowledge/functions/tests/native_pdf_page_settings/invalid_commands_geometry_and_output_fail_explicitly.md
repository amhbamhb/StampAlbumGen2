---
type: Rust Function
title: invalid_commands_geometry_and_output_fail_explicitly
resource: tests/native_pdf_page_settings.rs#L167-L202
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn invalid_commands_geometry_and_output_fail_explicitly() -> Result<()>`

# Calls

- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)
- [mm_to_points](../../../functions/src/native_pdf/units/mm_to_points.md)