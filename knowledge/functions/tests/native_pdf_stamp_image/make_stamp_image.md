---
type: Rust Function
title: make_stamp_image
resource: tests/native_pdf_stamp_image.rs#L41-L53
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_stamp_image(path: &Path) -> Result<()>`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)
- [save](../../../functions/src/review_ui/views/external_album/save.md)

# Called by

- [stamp_image_heading_caption_and_page_border_render_with_physical_geometry](../../../functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry.md)