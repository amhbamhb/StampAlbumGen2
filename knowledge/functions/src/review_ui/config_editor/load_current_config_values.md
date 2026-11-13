---
type: Rust Function
title: load_current_config_values
resource: src/review_ui/config_editor.rs#L507-L517
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/flatten_toml_value_into_map
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/update_default_catalog_code_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn load_current_config_values( config_path: &str, ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>`

# Calls

- [flatten_toml_value_into_map](../../../../functions/src/review_ui/config_editor/flatten_toml_value_into_map.md)

# Called by

- [update_default_catalog_code_for_album](../../../../functions/src/review_ui/config_editor/update_default_catalog_code_for_album.md)
- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [build_config_editor_data_for_album](../../../../functions/src/review_ui/config_editor/build_config_editor_data_for_album.md)
- [save_album_config_diff](../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)