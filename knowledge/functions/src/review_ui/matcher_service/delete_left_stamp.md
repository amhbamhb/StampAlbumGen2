---
type: Rust Function
title: delete_left_stamp
resource: src/review_ui/matcher_service.rs#L1544-L1593
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/restore_colnect_candidate_to_queue
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn delete_left_stamp( conn: &Connection, req: DeleteLeftStampRequest, ) -> DeleteLeftStampResponse`

# Calls

- [restore_colnect_candidate_to_queue](../../../../functions/src/review_ui/matcher_service/restore_colnect_candidate_to_queue.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)