---
type: Rust Function
title: mark_colnect_candidate_processed
resource: src/review_ui/matcher_service.rs#L25-L39
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/mark_colnect_candidate_processed_sets_flag_for_matching_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn mark_colnect_candidate_processed( conn: &Connection, colnect_id: &str, ) -> rusqlite::Result<usize>`

# Called by

- [mark_colnect_candidate_processed_sets_flag_for_matching_candidate](../../../../functions/src/review_ui/mark_colnect_candidate_processed_sets_flag_for_matching_candidate.md)
- [apply_matches](../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)