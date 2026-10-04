---
type: Rust Function
title: handle_country_url_request
resource: src/review_ui/http_handlers.rs#L217-L271
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/parse_query_param
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/load_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/save_country_url
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

`fn handle_country_url_request(mut request: tiny_http::Request) -> Option<tiny_http::Request>`

# Calls

- [parse_query_param](../../../../functions/src/review_ui/parse_query_param.md)
- [load_country_url](../../../../functions/src/stampworld/country_url/load_country_url.md)
- [save_country_url](../../../../functions/src/stampworld/country_url/save_country_url.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)