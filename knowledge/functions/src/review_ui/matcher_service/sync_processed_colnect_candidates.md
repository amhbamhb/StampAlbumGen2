---
type: Rust Function
title: sync_processed_colnect_candidates
resource: src/review_ui/matcher_service.rs#L5-L23
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/sync_processed_colnect_candidates_marks_existing_stamp_links
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn sync_processed_colnect_candidates( conn: &Connection, ) -> Result<(), Box<dyn std::error::Error>>`

# Called by

- [run_review_ui](../../../../functions/src/review_ui/run_review_ui.md)
- [sync_processed_colnect_candidates_marks_existing_stamp_links](../../../../functions/src/review_ui/sync_processed_colnect_candidates_marks_existing_stamp_links.md)