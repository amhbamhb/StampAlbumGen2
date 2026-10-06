---
type: Rust Function
title: numeric
resource: src/external_album/parser.rs#L3-L17
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/parser/number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn numeric(command: &LayoutCommand, index: usize, positive: bool) -> Result<f64>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [number](../../../../functions/src/external_album/parser/number.md)
- [validate](../../../../functions/src/external_album/parser/validate.md)