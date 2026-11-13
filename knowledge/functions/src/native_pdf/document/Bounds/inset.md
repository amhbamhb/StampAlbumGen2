---
type: Rust Method
title: inset
resource: src/native_pdf/document.rs#L34-L50
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_border/PageBorder/strokes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/proof/GeometryProofSettings/validate_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_box/validate_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/validate_numbers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/margin_and_even_page_bounds_match_frozen_expectations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn inset(self, horizontal_mm: f64, vertical_mm: f64) -> Result<Self>`

# Called by

- [strokes](../../../../../functions/src/native_pdf/commands/page/album_pages_border/PageBorder/strokes.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/layout.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/layout.md)
- [resolve_page](../../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)
- [validate_layout](../../../../../functions/src/native_pdf/proof/GeometryProofSettings/validate_layout.md)
- [validate_bounds](../../../../../functions/src/native_pdf/stamp_box/validate_bounds.md)
- [validate_numbers](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/validate_numbers.md)
- [prepare](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [margin_and_even_page_bounds_match_frozen_expectations](../../../../../functions/tests/native_pdf_page_settings/margin_and_even_page_bounds_match_frozen_expectations.md)