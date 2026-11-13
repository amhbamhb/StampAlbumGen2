---
type: Rust Function
title: inspect
resource: tests/albumeasy_baseline.rs#L231-L250
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/tool_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/inspect_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn inspect(pdf: &Path, expected: &Expectations) -> Result<String>`

# Calls

- [tool_output](../../../functions/tests/albumeasy_baseline/tool_output.md)
- [check_trace](../../../functions/tests/albumeasy_baseline/check_trace.md)
- [inspect_text](../../../functions/tests/albumeasy_baseline/inspect_text.md)