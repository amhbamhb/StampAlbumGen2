---
type: Rust Function
title: candidate_matches_reference_strictly
resource: tests/albumeasy_baseline.rs#L372-L380
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/check_frozen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/expectations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_strict_trace
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn candidate_matches_reference_strictly() -> Result<()>`

# Calls

- [check_frozen](../../../functions/tests/albumeasy_baseline/check_frozen.md)
- [expectations](../../../functions/tests/albumeasy_baseline/expectations.md)
- [check_strict_trace](../../../functions/tests/albumeasy_baseline/check_strict_trace.md)