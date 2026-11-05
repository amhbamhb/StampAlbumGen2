---
type: Rust Function
title: write_page_geometry_proof_with_date
resource: src/native_pdf/renderer.rs#L21-L215
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
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_geometry_proof_with_date( settings: &PageSettings, proof: &GeometryProofSettings, output: &Path, generation_date: Option<chrono::NaiveDate>, ) -> Result<()>`

# Calls

- [resolve_with_context](../../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)
- [validate_layout](../../../../functions/src/native_pdf/proof/GeometryProofSettings/validate_layout.md)
- [mm_to_points](../../../../functions/src/native_pdf/units/mm_to_points.md)
- [width_mm](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [border_line_colour](../../../../functions/src/native_pdf/document/PageSettings/border_line_colour.md)
- [footprints](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderLayout/footprints.md)
- [draw_text_lines](../../../../functions/src/native_pdf/renderer/draw_text_lines.md)
- [save](../../../../functions/src/review_ui/views/external_album/save.md)

# Called by

- [write_page_geometry_proof](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [generation_date_requires_one_explicit_document_context](../../../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)