---
type: Rust Function
title: validate_image_rect
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L403-L413
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_image_rect([x, y, width, height]: [f64; 4]) -> Result<()>`

# Calls

- [mm_to_points](../../../../../../functions/src/native_pdf/units/mm_to_points.md)

# Called by

- [layout](../../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout.md)