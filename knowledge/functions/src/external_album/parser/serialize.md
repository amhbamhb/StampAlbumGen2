---
type: Rust Function
title: serialize
resource: src/external_album/parser.rs#L345-L369
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn serialize(globals: &[LayoutCommand], pages: &[LayoutPage]) -> Result<String>`

# Calls

- [validate_document](../../../../functions/src/external_album/parser/validate_document.md)

# Called by

- [render_pair](../../../../functions/src/external_album/render_pair.md)
- [verify_layout](../../../../functions/src/external_album/verify_layout.md)
- [export_layout](../../../../functions/src/external_album/export_layout.md)