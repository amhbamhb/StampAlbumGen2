---
type: Rust Function
title: clear_left_colnect_link
resource: src/review_ui/matcher_service.rs#L1595-L1696
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/restore_colnect_candidate_to_queue
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn clear_left_colnect_link( conn: &Connection, db_path: &str, album_name: &str, req: ClearLeftColnectLinkRequest, ) -> ClearLeftColnectLinkResponse`

# Calls

- [restore_colnect_candidate_to_queue](../../../../functions/src/review_ui/matcher_service/restore_colnect_candidate_to_queue.md)
- [load_left_stamp_by_rowid](../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)