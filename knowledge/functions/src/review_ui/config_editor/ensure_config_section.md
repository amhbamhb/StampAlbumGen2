---
type: Rust Function
title: ensure_config_section
resource: src/review_ui/config_editor.rs#L410-L426
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/friendly_config_section_title
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/parse_template_config_sections
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ensure_config_section<'a>( sections: &'a mut Vec<ConfigEditorSection>, section_id: &str, section_comments: Vec<String>, ) -> &'a mut ConfigEditorSection`

# Calls

- [friendly_config_section_title](../../../../functions/src/review_ui/config_editor/friendly_config_section_title.md)

# Called by

- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)