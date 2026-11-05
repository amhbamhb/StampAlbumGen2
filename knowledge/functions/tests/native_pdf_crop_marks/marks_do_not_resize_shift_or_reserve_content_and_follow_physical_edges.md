---
type: Rust Function
title: marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges
resource: tests/native_pdf_crop_marks.rs#L135-L167
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_crop_marks/config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges() -> Result<()>`

# Calls

- [config](../../../functions/tests/native_pdf_crop_marks/config.md)
- [width_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)