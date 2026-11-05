---
type: Rust Function
title: strokes
resource: tests/native_pdf_crop_marks.rs#L466-L518
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strokes(xml: &str) -> Result<Vec<Vec<f64>>>`

# Calls

- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)