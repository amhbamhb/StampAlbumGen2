---
type: Rust Function
title: store_uploaded_image_from_data_url
resource: src/review_ui.rs#L2789-L2843
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/ext_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/safe_filename_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/resolve_left_editor_image_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn store_uploaded_image_from_data_url( db_path: &str, album_name: &str, colnect_id: &str, upload_data_url: &str, upload_filename: Option<&str>, ) -> Result<String, String>`

# Calls

- [ext_from_data_url](../../../functions/src/review_ui/ext_from_data_url.md)
- [safe_filename_token](../../../functions/src/review_ui/safe_filename_token.md)
- [album_dir_for_ui](../../../functions/src/review_ui/album_dir_for_ui.md)

# Called by

- [add_colnect_stamp_to_stamps](../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [resolve_left_editor_image_path](../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)