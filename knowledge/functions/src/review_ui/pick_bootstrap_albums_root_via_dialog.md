---
type: Rust Function
title: pick_bootstrap_albums_root_via_dialog
resource: src/review_ui.rs#L1980-L1987
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/current_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/windows_picker/pick
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_bootstrap_albums_root_via_dialog() -> Result<Option<String>, String>`

# Calls

- [current_bootstrap_albums_root](../../../functions/src/review_ui/current_bootstrap_albums_root.md)
- [pick](../../../functions/src/review_ui/windows_picker/pick.md)