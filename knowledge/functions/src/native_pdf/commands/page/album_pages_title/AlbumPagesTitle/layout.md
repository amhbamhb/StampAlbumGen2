---
type: Rust Method
title: layout
resource: src/native_pdf/commands/page/album_pages_title.rs#L64-L124
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
---

# Signature

`pub fn layout(&self, content: Bounds, vertical_spacing_mm: f64) -> Result<TitleLayout>`

# Calls

- [inset](../../../../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [points_to_mm](../../../../../../../functions/src/native_pdf/units/points_to_mm.md)