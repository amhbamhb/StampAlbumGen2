---
type: Rust Method
title: layout
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L241-L353
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/validate_image_rect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn layout(&self, margin: Bounds, page_height_mm: f64) -> Result<DecorativeBorderLayout>`

# Calls

- [inset](../../../../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [mm_to_points](../../../../../../../functions/src/native_pdf/units/mm_to_points.md)
- [image_dimensions_mm](../../../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm.md)
- [validate_image_rect](../../../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/validate_image_rect.md)