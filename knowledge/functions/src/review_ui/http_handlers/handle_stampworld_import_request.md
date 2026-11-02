---
type: Rust Function
title: handle_stampworld_import_request
resource: src/review_ui/http_handlers.rs#L26-L107
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_to_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/respond
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_stampworld_import_request( mut request: tiny_http::Request, review: Option<(&Connection, &str, &str)>, ) -> Option<tiny_http::Request>`

# Calls

- [scan_bootstrap_countries](../../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [pick_and_copy_colnect_file_to_dir](../../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)
- [import_stampworld_file](../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)
- [respond](../../../../functions/src/review_ui/external_album_api/respond.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)