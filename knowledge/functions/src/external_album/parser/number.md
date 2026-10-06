---
type: Rust Function
title: number
resource: src/external_album/parser.rs#L19-L21
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/numeric
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/reflow/reflow
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn number(command: &LayoutCommand, index: usize) -> Result<f64>`

# Calls

- [numeric](../../../../functions/src/external_album/parser/numeric.md)

# Called by

- [reflow](../../../../functions/src/external_album/reflow/reflow.md)