---
type: Rust Function
title: delete_colnect_candidate
resource: src/review_ui/matcher_service.rs#L1674-L1715
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn delete_colnect_candidate( conn: &Connection, req: DeleteColnectCandidateRequest, ) -> DeleteColnectCandidateResponse`

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)