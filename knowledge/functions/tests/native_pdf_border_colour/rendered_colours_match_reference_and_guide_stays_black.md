---
type: Rust Function
title: rendered_colours_match_reference_and_guide_stays_black
resource: tests/native_pdf_border_colour.rs#L213-L337
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/source_colour
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_border_colour
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rendered_colours_match_reference_and_guide_stays_black() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [source_colour](../../../functions/tests/native_pdf_border_colour/source_colour.md)
- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)
- [set_border_colour](../../../functions/src/native_pdf/document/PageSettings/set_border_colour.md)
- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)