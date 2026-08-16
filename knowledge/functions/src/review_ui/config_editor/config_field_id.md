---
type: Rust Function
title: config_field_id
resource: src/review_ui/config_editor.rs#L54-L60
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/config_editor/load_example_config_help_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/parse_template_config_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn config_field_id(section_id: &str, key: &str) -> String`

# Called by

- [load_example_config_help_map](../../../../functions/src/review_ui/config_editor/load_example_config_help_map.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [render_config_from_template_values](../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)