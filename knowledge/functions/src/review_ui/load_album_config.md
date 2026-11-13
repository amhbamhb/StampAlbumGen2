---
type: Rust Function
title: load_album_config
resource: src/review_ui.rs#L2758-L2761
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_album_config(db_path: &str, album_name: &str) -> crate::config::Config`

# Calls

- [album_dir_for_ui](../../../functions/src/review_ui/album_dir_for_ui.md)
- [load_config_with_override](../../../functions/src/config/load_config_with_override.md)

# Called by

- [colnect_import_data](../../../functions/src/review_ui/colnect_import_data.md)
- [apply_matches](../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [add_left_stamp](../../../functions/src/review_ui/matcher_service/add_left_stamp.md)