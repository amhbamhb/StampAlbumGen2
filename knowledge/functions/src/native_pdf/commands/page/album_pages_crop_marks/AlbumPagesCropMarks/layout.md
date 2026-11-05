---
type: Rust Method
title: layout
resource: src/native_pdf/commands/page/album_pages_crop_marks.rs#L54-L97
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn layout(self, size: PageSize) -> Result<CropMarksLayout>`

# Calls

- [width_mm](../../../../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [mm_to_points](../../../../../../../functions/src/native_pdf/units/mm_to_points.md)