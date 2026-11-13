---
type: Rust Function
title: check_trace
resource: tests/albumeasy_baseline.rs#L73-L175
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/coordinate
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/albumeasy_baseline/inspect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_trace(trace: &str, expected: &Expectations) -> Result<()>`

# Calls

- [coordinate](../../../functions/tests/albumeasy_baseline/coordinate.md)

# Called by

- [inspect](../../../functions/tests/albumeasy_baseline/inspect.md)