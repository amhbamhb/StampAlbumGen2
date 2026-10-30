---
type: Rust Function
title: picker_command
resource: src/review_ui/windows_picker.rs#L10-L38
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/windows_picker/pick
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/windows_picker/windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn picker_command(kind: PickerKind, title: &str, start: &str) -> Command`

# Called by

- [pick](../../../../functions/src/review_ui/windows_picker/pick.md)
- [windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values](../../../../functions/src/review_ui/windows_picker/windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values.md)