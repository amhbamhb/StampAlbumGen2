---
type: Rust Function
title: check_text
resource: tests/albumeasy_baseline.rs#L177-L185
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/albumeasy_baseline/inspect_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/tolerance_and_missing_text_are_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_text(text: &str, required: &[String], page: usize) -> Result<()>`

# Called by

- [inspect_text](../../../functions/tests/albumeasy_baseline/inspect_text.md)
- [tolerance_and_missing_text_are_rejected](../../../functions/tests/albumeasy_baseline/tolerance_and_missing_text_are_rejected.md)