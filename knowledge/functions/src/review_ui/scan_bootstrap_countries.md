---
type: Rust Function
title: scan_bootstrap_countries
resource: src/review_ui.rs#L1886-L1939
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_colnect_csv_files_rec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_import_files_rec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_colnect_candidates_in_db
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_stampworld_import_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scan_bootstrap_countries() -> Vec<BootstrapCountryInfo>`

# Calls

- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [count_colnect_csv_files_rec](../../../functions/src/review_ui/count_colnect_csv_files_rec.md)
- [count_import_files_rec](../../../functions/src/review_ui/count_import_files_rec.md)
- [count_colnect_candidates_in_db](../../../functions/src/review_ui/count_colnect_candidates_in_db.md)

# Called by

- [pick_and_copy_stampworld_file_via_dialog](../../../functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog.md)
- [pick_and_copy_colnect_file_via_dialog](../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [handle_stampworld_import_request](../../../functions/src/review_ui/http_handlers/handle_stampworld_import_request.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)