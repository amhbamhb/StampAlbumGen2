---
type: Rust Function
title: resolve_image_abs
resource: src/review_ui.rs#L2086-L2105
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_image_abs(db_path: &str, album_name: &str, rel_image_path: &str) -> String`

# Calls

- [resolve_album_image_path](../../../functions/src/file_utils/resolve_album_image_path.md)

# Called by

- [load_left_stamps](../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [load_left_stamp_by_rowid](../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)
- [load_right_candidate_by_colnect_id](../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)