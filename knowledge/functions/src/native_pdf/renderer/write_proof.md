---
type: Rust Function
title: write_proof
resource: src/native_pdf/renderer.rs#L74-L315
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/proof/GeometryProofSettings/validate_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/border_line_colour
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderLayout/footprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/draw_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_image_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_stamp_box_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_proof( settings: &PageSettings, proof: &GeometryProofSettings, output: &Path, generation_date: Option<chrono::NaiveDate>, stamp: Option<&StampImageProofSettings>, geometry: Option<&StampBoxGeometry>, ) -> Result<()>`

# Calls

- [resolve_with_context](../../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)
- [validate_layout](../../../../functions/src/native_pdf/proof/GeometryProofSettings/validate_layout.md)
- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [mm_to_points](../../../../functions/src/native_pdf/units/mm_to_points.md)
- [width_mm](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [border_line_colour](../../../../functions/src/native_pdf/document/PageSettings/border_line_colour.md)
- [footprints](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderLayout/footprints.md)
- [draw_text_lines](../../../../functions/src/native_pdf/renderer/draw_text_lines.md)
- [frame_bounds](../../../../functions/src/native_pdf/stamp_box/StampBoxGeometry/frame_bounds.md)
- [save](../../../../functions/src/review_ui/views/external_album/save.md)

# Called by

- [write_page_geometry_proof_with_date](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [write_stamp_image_proof](../../../../functions/src/native_pdf/renderer/write_stamp_image_proof.md)
- [write_stamp_box_geometry_proof](../../../../functions/src/native_pdf/renderer/write_stamp_box_geometry_proof.md)