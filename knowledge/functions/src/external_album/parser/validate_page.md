---
type: Rust Function
title: validate_page
resource: src/external_album/parser.rs#L141-L171
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn validate_page(page: &LayoutPage) -> Result<()>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [edit_page](../../../../functions/src/external_album/edit_page.md)
- [validate_document](../../../../functions/src/external_album/parser/validate_document.md)