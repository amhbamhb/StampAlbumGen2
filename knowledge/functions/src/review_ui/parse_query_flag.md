---
type: Rust Function
title: parse_query_flag
resource: src/review_ui.rs#L2512-L2524
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/parse_query_param
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_query_flag(url: &str, key: &str) -> bool`

# Calls

- [parse_query_param](../../../functions/src/review_ui/parse_query_param.md)

# Called by

- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)