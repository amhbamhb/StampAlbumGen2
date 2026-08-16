---
type: Rust Function
title: undo_colnect_delete
resource: src/review_ui/matcher_service.rs#L1709-L1747
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn undo_colnect_delete( conn: &Connection, req: UndoColnectDeleteRequest, ) -> UndoColnectDeleteResponse`

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)