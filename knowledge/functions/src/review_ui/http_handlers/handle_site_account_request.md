---
type: Rust Function
title: handle_site_account_request
resource: src/review_ui/http_handlers.rs#L154-L215
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/save_site_credentials
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
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

`fn handle_site_account_request(mut request: tiny_http::Request) -> Option<tiny_http::Request>`

# Calls

- [save_site_credentials](../../../../functions/src/review_ui/save_site_credentials.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)