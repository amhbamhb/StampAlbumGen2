---
type: Rust Function
title: pick_albumeasy_cli_path_via_dialog
resource: src/review_ui.rs#L2078-L2088
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/windows_picker/pick
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_albumeasy_cli_path_via_dialog(current_path: Option<&str>) -> Result<Option<String>, String>`

# Calls

- [pick](../../../functions/src/review_ui/windows_picker/pick.md)