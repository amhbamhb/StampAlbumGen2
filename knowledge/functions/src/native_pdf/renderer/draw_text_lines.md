---
type: Rust Function
title: draw_text_lines
resource: src/native_pdf/renderer.rs#L217-L235
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn draw_text_lines( page: &mut Page, font: oxidize_pdf::text::Font, size_pt: f64, colour: NamedColour, lines: Vec<PositionedTextLine>, kind: &str, ) -> Result<()>`

# Calls

- [mm_to_points](../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [write_page_geometry_proof_with_date](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)