---
type: Rust Function
title: restore_colnect_candidate_to_queue
resource: src/review_ui/matcher_service.rs#L1105-L1119
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/matcher_service/delete_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/clear_left_colnect_link
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn restore_colnect_candidate_to_queue( conn: &Connection, colnect_id: &str, ) -> rusqlite::Result<usize>`

# Called by

- [delete_left_stamp](../../../../functions/src/review_ui/matcher_service/delete_left_stamp.md)
- [clear_left_colnect_link](../../../../functions/src/review_ui/matcher_service/clear_left_colnect_link.md)