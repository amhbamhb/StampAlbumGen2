---
type: Rust Function
title: query_stamp_image_path
resource: src/review_ui/matcher_service.rs#L1144-L1151
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/matcher_service/resolve_left_editor_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/save_left_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn query_stamp_image_path(conn: &Connection, rowid: i64) -> Result<String, String>`

# Called by

- [resolve_left_editor_image_path](../../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)
- [save_left_stamp](../../../../functions/src/review_ui/matcher_service/save_left_stamp.md)