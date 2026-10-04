---
type: Rust Function
title: parse_query_param
resource: src/review_ui.rs#L2490-L2499
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/parse_query_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_query_param(url: &str, key: &str) -> Option<String>`

# Called by

- [parse_query_flag](../../../functions/src/review_ui/parse_query_flag.md)
- [handle_country_url_request](../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)