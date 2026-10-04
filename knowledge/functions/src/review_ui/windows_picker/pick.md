---
type: Rust Function
title: pick
resource: src/review_ui/windows_picker.rs#L62-L67
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
  called_by:
  - target: functions/src/review_ui/pick_bootstrap_albums_root_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_albumeasy_cli_path_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_to_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn pick(kind: PickerKind, title: &str, start: &str) -> Result<Option<String>, String>`

# Calls

- [picker_command](../../../../functions/src/review_ui/windows_picker/picker_command.md)
- [decode_selection](../../../../functions/src/review_ui/windows_picker/decode_selection.md)

# Called by

- [pick_bootstrap_albums_root_via_dialog](../../../../functions/src/review_ui/pick_bootstrap_albums_root_via_dialog.md)
- [pick_albumeasy_cli_path_via_dialog](../../../../functions/src/review_ui/pick_albumeasy_cli_path_via_dialog.md)
- [pick_and_copy_colnect_file_to_dir](../../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)