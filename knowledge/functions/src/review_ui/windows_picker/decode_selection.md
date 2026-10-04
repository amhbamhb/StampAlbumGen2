---
type: Rust Function
title: decode_selection
resource: src/review_ui/windows_picker.rs#L38-L59
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/windows_picker/pick
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/windows_picker/windows_picker_distinguishes_cancellation_from_failure
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn decode_selection( status: Option<i32>, stdout: &[u8], stderr: &[u8], ) -> Result<Option<String>, String>`

# Called by

- [pick](../../../../functions/src/review_ui/windows_picker/pick.md)
- [windows_picker_distinguishes_cancellation_from_failure](../../../../functions/src/review_ui/windows_picker/windows_picker_distinguishes_cancellation_from_failure.md)