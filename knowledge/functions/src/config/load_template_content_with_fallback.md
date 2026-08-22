---
type: Rust Function
title: load_template_content_with_fallback
resource: src/config.rs#L30-L37
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_template_content_with_fallback( template_path: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Called by

- [build_config_editor_data](../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [save_full_config_values](../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [save_album_config_diff](../../../functions/src/review_ui/config_editor/save_album_config_diff.md)