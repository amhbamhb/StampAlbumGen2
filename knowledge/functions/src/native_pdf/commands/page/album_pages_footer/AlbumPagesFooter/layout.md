---
type: Rust Method
title: layout
resource: src/native_pdf/commands/page/album_pages_footer.rs#L62-L135
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/text/TextPosition/line_left
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn layout( &self, margin: Bounds, physical_page: Bounds, page_number: usize, ) -> Result<FooterLayout>`

# Calls

- [inset](../../../../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [points_to_mm](../../../../../../../functions/src/native_pdf/units/points_to_mm.md)
- [line_left](../../../../../../../functions/src/native_pdf/text/TextPosition/line_left.md)