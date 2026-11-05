---
type: Rust Function
title: validate
resource: src/external_album/parser.rs#L23-L139
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/numeric
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn validate(command: &LayoutCommand) -> Result<()>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [numeric](../../../../functions/src/external_album/parser/numeric.md)