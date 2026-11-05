---
type: Rust Function
title: oxidize_reference_text_and_negative_controls
resource: tests/albumeasy_baseline.rs#L293-L308
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/check_frozen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/inspect_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/expectations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn oxidize_reference_text_and_negative_controls() -> Result<()>`

# Calls

- [check_frozen](../../../functions/tests/albumeasy_baseline/check_frozen.md)
- [inspect_text](../../../functions/tests/albumeasy_baseline/inspect_text.md)
- [expectations](../../../functions/tests/albumeasy_baseline/expectations.md)