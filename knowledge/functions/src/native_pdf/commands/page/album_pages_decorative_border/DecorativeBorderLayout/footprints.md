---
type: Rust Method
title: footprints
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L87-L118
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/mirrored_corner_bounds_and_fixed_slot_line_spacing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn footprints(&self) -> Vec<Bounds>`

# Called by

- [write_page_geometry_proof_with_date](../../../../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [mirrored_corner_bounds_and_fixed_slot_line_spacing](../../../../../../../functions/tests/native_pdf_decorative_border/mirrored_corner_bounds_and_fixed_slot_line_spacing.md)