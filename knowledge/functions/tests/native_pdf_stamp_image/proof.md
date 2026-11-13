---
type: Rust Function
title: proof
resource: tests/native_pdf_stamp_image.rs#L32-L39
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn proof() -> Result<GeometryProofSettings>`

# Called by

- [stamp_image_heading_caption_and_page_border_render_with_physical_geometry](../../../functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)