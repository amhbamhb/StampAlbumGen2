---
type: Rust Function
title: inspect_text
resource: tests/albumeasy_baseline.rs#L201-L229
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/albumeasy_baseline/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn inspect_text(pdf: &Path, expected: &Expectations) -> Result<()>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [check_text](../../../functions/tests/albumeasy_baseline/check_text.md)

# Called by

- [inspect](../../../functions/tests/albumeasy_baseline/inspect.md)
- [oxidize_reference_text_and_negative_controls](../../../functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls.md)