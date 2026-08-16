---
type: Rust Function
title: resolve_left_editor_image_path
resource: src/review_ui/matcher_service.rs#L1121-L1167
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/query_stamp_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/save_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn resolve_left_editor_image_path( conn: &Connection, db_path: &str, album_name: &str, image_mode: Option<&str>, source_rowid: Option<i64>, current_image_path: Option<&str>, upload_data_url: Option<&str>, upload_filename: Option<&str>, file_token: &str, ) -> Result<String, String>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [query_stamp_image_path](../../../../functions/src/review_ui/matcher_service/query_stamp_image_path.md)
- [store_uploaded_image_from_data_url](../../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)

# Called by

- [save_left_stamp](../../../../functions/src/review_ui/matcher_service/save_left_stamp.md)
- [add_left_stamp](../../../../functions/src/review_ui/matcher_service/add_left_stamp.md)