---
type: Rust Function
title: pick
resource: src/review_ui/windows_picker.rs#L64-L69
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/windows_picker/picker_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/windows_picker/decode_selection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn pick(kind: PickerKind, title: &str, start: &str) -> Result<Option<String>, String>`

# Calls

- [picker_command](../../../../functions/src/review_ui/windows_picker/picker_command.md)
- [decode_selection](../../../../functions/src/review_ui/windows_picker/decode_selection.md)