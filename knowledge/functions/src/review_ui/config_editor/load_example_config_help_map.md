---
type: Rust Function
title: load_example_config_help_map
resource: src/review_ui/config_editor.rs#L101-L153
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/progress/ProgressIndicator/clear
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
---

# Signature

`fn load_example_config_help_map(path: &str) -> HashMap<String, Vec<String>>`

# Calls

- [clear](../../../../functions/src/progress/ProgressIndicator/clear.md)
- [parse_config_key_value_line](../../../../functions/src/review_ui/config_editor/parse_config_key_value_line.md)
- [config_field_id](../../../../functions/src/review_ui/config_editor/config_field_id.md)

# Called by

- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)