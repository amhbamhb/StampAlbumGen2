---
type: Rust Function
title: best_effort_prepare_images_for_pdf
resource: src/review_ui/db_query.rs#L6-L11
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn best_effort_prepare_images_for_pdf( conn: &Connection, album_name: &str, ) -> (usize, usize)`

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)