---
type: Rust Function
title: respond
resource: src/review_ui/external_album_api.rs#L431-L447
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_site_account_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn respond(request: tiny_http::Request, result: Result<Value, String>, error_status: u16)`

# Calls

- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)
- [handle_site_account_request](../../../../functions/src/review_ui/http_handlers/handle_site_account_request.md)
- [handle_country_url_request](../../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)
- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)