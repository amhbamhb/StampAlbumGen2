---
type: Rust Function
title: resolve_colnect_image_url
resource: src/colnect/image_fetcher.rs#L163-L170
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_colnect_image_url(page_url: &str) -> Result<String, Box<dyn std::error::Error>>`

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [load_right_candidate_by_colnect_id](../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)