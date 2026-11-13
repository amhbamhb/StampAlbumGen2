---
type: Rust Function
title: write_stamp_image_proof
resource: src/native_pdf/renderer.rs#L32-L49
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/capture_generation_date
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_stamp_image_proof( settings: &PageSettings, proof: &GeometryProofSettings, stamp: &StampImageProofSettings, output: &Path, ) -> Result<()>`

# Calls

- [write_proof](../../../../functions/src/native_pdf/renderer/write_proof.md)
- [capture_generation_date](../../../../functions/src/native_pdf/document/PageSettings/capture_generation_date.md)

# Called by

- [stamp_image_heading_caption_and_page_border_render_with_physical_geometry](../../../../functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry.md)