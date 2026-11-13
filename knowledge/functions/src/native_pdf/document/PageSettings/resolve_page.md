---
type: Rust Method
title: resolve_page
resource: src/native_pdf/document.rs#L262-L472
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_border/PageBorder/occupancy_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/text/text_lines_overlap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/document/PageSettings/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_page( &self, page_number: usize, total_pages: Option<usize>, generation_date: Option<chrono::NaiveDate>, ) -> Result<PageLayout>`

# Calls

- [width_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/width_mm.md)
- [height_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/height_mm.md)
- [occupancy_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_border/PageBorder/occupancy_mm.md)
- [inset](../../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [horizontal_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/horizontal_mm.md)
- [text_lines_overlap](../../../../../functions/src/native_pdf/text/text_lines_overlap.md)
- [vertical_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/vertical_mm.md)

# Called by

- [resolve](../../../../../functions/src/native_pdf/document/PageSettings/resolve.md)
- [resolve_with_context](../../../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)