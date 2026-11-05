---
type: Rust Method
title: line_left
resource: src/native_pdf/text.rs#L34-L45
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn line_left(self, page_number: usize, left: f64, right: f64, width: f64) -> f64`

# Called by

- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/layout.md)