---
type: Rust Function
title: image_dimensions_mm
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L382-L401
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn image_dimensions_mm(image: &Image, scale: f64) -> Result<(f64, f64)>`

# Calls

- [from](../../../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [points_to_mm](../../../../../../functions/src/native_pdf/units/points_to_mm.md)
- [mm_to_points](../../../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [layout](../../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout.md)