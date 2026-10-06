---
type: Rust Function
title: setting
resource: src/external_album/reflow.rs#L3-L9
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/external_album/reflow/reflow
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn setting<'a>(globals: &'a [LayoutCommand], name: &str) -> Result<&'a LayoutCommand>`

# Called by

- [reflow](../../../../functions/src/external_album/reflow/reflow.md)