---
type: Rust Function
title: validate_document
resource: src/external_album/parser.rs#L155-L184
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/validate_page
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/serialize
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn validate_document(globals: &[LayoutCommand], pages: &[LayoutPage]) -> Result<()>`

# Calls

- [validate_page](../../../../functions/src/external_album/parser/validate_page.md)

# Called by

- [details](../../../../functions/src/external_album/details.md)
- [edit_page](../../../../functions/src/external_album/edit_page.md)
- [parse](../../../../functions/src/external_album/parser/parse.md)
- [serialize](../../../../functions/src/external_album/parser/serialize.md)