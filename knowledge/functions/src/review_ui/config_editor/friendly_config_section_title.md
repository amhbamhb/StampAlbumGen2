---
type: Rust Function
title: friendly_config_section_title
resource: src/review_ui/config_editor.rs#L62-L86
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/config_editor/ensure_config_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn friendly_config_section_title(section_id: &str) -> String`

# Called by

- [ensure_config_section](../../../../functions/src/review_ui/config_editor/ensure_config_section.md)
- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)