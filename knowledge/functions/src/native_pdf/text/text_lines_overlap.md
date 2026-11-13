---
type: Rust Function
title: text_lines_overlap
resource: src/native_pdf/text.rs#L63-L97
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/document/PageSettings/resolve_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn text_lines_overlap( a: &[PositionedTextLine], a_bounds: Bounds, b: &[PositionedTextLine], b_bounds: Bounds, ) -> Result<bool>`

# Called by

- [resolve_page](../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)