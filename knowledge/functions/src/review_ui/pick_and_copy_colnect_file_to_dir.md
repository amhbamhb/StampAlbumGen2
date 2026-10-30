---
type: Rust Function
title: pick_and_copy_colnect_file_to_dir
resource: src/review_ui.rs#L2237-L2367
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/is_stampworld_import_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_for_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_stampworld_import_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_and_copy_colnect_file_to_dir<F>( colnect_dir: &Path, title: &str, colnect_dir_str: &str, stampworld: bool, fail: F, ) -> BootstrapPickColnectFileResponse where F: Fn(String) -> BootstrapPickColnectFileResponse,`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [is_stampworld_import_file](../../../functions/src/review_ui/is_stampworld_import_file.md)

# Called by

- [pick_and_copy_stampworld_file_via_dialog](../../../functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog.md)
- [pick_and_copy_colnect_file_via_dialog](../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [pick_and_copy_colnect_file_for_db](../../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)
- [handle_stampworld_import_request](../../../functions/src/review_ui/http_handlers/handle_stampworld_import_request.md)