---
type: Rust Method
title: is_all
resource: src/album_globals/selection.rs#L35-L37
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_all(&self) -> bool`

# Called by

- [generate_album_output_conn_with_filters](../../../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [serve_review_requests](../../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)