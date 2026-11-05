---
type: Rust Function
title: mm_to_points
resource: src/native_pdf/units.rs#L1-L3
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_border/PageBorder/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderPolicy/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/validate_image_rect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_footer_pad/FooterPadding/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_header_pad/HeaderPadding/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/proof/GeometryProofSettings/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/draw_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/text/MeasuredText/new_with_font
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn mm_to_points(mm: f64) -> f64`

# Called by

- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_border/PageBorder/parse.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/parse.md)
- [layout](../../../../functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/layout.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderPolicy/parse.md)
- [layout](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout.md)
- [image_dimensions_mm](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm.md)
- [validate_image_rect](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/validate_image_rect.md)
- [new](../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/new.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_footer_pad/FooterPadding/parse.md)
- [new](../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/new.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_header_pad/HeaderPadding/parse.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/parse.md)
- [new](../../../../functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/new.md)
- [validate](../../../../functions/src/native_pdf/proof/GeometryProofSettings/validate.md)
- [write_page_geometry_proof_with_date](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [draw_text_lines](../../../../functions/src/native_pdf/renderer/draw_text_lines.md)
- [new_with_font](../../../../functions/src/native_pdf/text/MeasuredText/new_with_font.md)
- [invalid_commands_geometry_and_output_fail_explicitly](../../../../functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly.md)
- [rendered_paths_match_margin_and_spacing_fixtures](../../../../functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures.md)