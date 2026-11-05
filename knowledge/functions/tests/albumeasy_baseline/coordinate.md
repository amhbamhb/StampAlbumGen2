---
type: Rust Function
title: coordinate
resource: tests/albumeasy_baseline.rs#L65-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/albumeasy_baseline/check_trace
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn coordinate(node: Node<'_, '_>, name: &str) -> Result<f64>`

# Called by

- [check_trace](../../../functions/tests/albumeasy_baseline/check_trace.md)