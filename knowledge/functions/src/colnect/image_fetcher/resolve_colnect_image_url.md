---
type: Rust Function
title: resolve_colnect_image_url
resource: src/colnect/image_fetcher.rs#L146-L150
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/build_client
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/extract_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_colnect_image_url(page_url: &str) -> Option<String>`

# Calls

- [build_client](../../../../functions/src/stampworld/fetch/build_client.md)
- [extract_colnect_image_url](../../../../functions/src/colnect/image_fetcher/extract_colnect_image_url.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [load_right_candidate_by_colnect_id](../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)