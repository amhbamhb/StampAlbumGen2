---
type: Rust Function
title: parse_template_config_sections
resource: src/review_ui/config_editor.rs#L418-L473
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/ensure_config_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/parse_config_key_value_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/config_field_id
    resolved_by: tree-sitter
    confidence: exact
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

`fn parse_template_config_sections(template_content: &str) -> Vec<ConfigEditorSection>`

# Calls

- [clear](../../../../functions/src/progress/ProgressIndicator/clear.md)
- [ensure_config_section](../../../../functions/src/review_ui/config_editor/ensure_config_section.md)
- [parse_config_key_value_line](../../../../functions/src/review_ui/config_editor/parse_config_key_value_line.md)
- [config_field_id](../../../../functions/src/review_ui/config_editor/config_field_id.md)

# Called by

- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [save_full_config_values](../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [save_album_config_diff](../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)