---
type: Rust Function
title: handle
resource: src/review_ui/external_album_api.rs#L394-L429
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/external_album_api/same_origin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/respond
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/pdf_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/parameter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn handle(mut request: tiny_http::Request) -> Option<tiny_http::Request>`

# Calls

- [same_origin](../../../../functions/src/review_ui/external_album_api/same_origin.md)
- [respond](../../../../functions/src/review_ui/external_album_api/respond.md)
- [root](../../../../functions/src/review_ui/external_album_api/root.md)
- [pdf_path](../../../../functions/src/review_ui/external_album_api/pdf_path.md)
- [parameter](../../../../functions/src/review_ui/external_album_api/parameter.md)
- [open](../../../../functions/src/external_album/open.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)
- [route](../../../../functions/src/review_ui/external_album_api/route.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)