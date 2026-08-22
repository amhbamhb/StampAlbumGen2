---
type: Rust Function
title: pick_and_copy_colnect_file_to_dir
resource: src/review_ui.rs#L1860-L1966
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pick_and_copy_colnect_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_for_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_and_copy_colnect_file_to_dir<F>( colnect_dir: &Path, title: &str, colnect_dir_str: &str, fail: F, ) -> BootstrapPickColnectFileResponse where F: Fn(String) -> BootstrapPickColnectFileResponse,`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [pick_and_copy_colnect_file_via_dialog](../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [pick_and_copy_colnect_file_for_db](../../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)