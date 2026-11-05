---
type: Rust Method
title: new_with_font
resource: src/native_pdf/text.rs#L134-L178
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/text/MeasuredText/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/text/MeasuredText/with_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn new_with_font(font: Font, size_pt: f64, text: &str) -> Result<Self>`

# Calls

- [points_to_mm](../../../../../functions/src/native_pdf/units/points_to_mm.md)
- [mm_to_points](../../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [new](../../../../../functions/src/native_pdf/text/MeasuredText/new.md)
- [with_text](../../../../../functions/src/native_pdf/text/MeasuredText/with_text.md)