---
type: Rust Function
title: page_traces
resource: tests/albumeasy_baseline.rs#L252-L259
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/albumeasy_baseline/check_strict_trace
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn page_traces(trace: &str) -> Result<Vec<String>>`

# Called by

- [check_strict_trace](../../../functions/tests/albumeasy_baseline/check_strict_trace.md)