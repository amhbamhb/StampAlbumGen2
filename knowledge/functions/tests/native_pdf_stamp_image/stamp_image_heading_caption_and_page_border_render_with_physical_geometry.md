---
type: Rust Function
title: stamp_image_heading_caption_and_page_border_render_with_physical_geometry
resource: tests/native_pdf_stamp_image.rs#L56-L107
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_stamp_image/make_stamp_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_image_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stamp_image_heading_caption_and_page_border_render_with_physical_geometry() -> Result<()>`

# Calls

- [make_stamp_image](../../../functions/tests/native_pdf_stamp_image/make_stamp_image.md)
- [write_stamp_image_proof](../../../functions/src/native_pdf/renderer/write_stamp_image_proof.md)
- [proof](../../../functions/tests/native_pdf_stamp_image/proof.md)
- [open](../../../functions/src/external_album/open.md)