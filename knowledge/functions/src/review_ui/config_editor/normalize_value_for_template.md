---
type: Rust Function
title: normalize_value_for_template
resource: src/review_ui/config_editor.rs#L349-L401
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/template_literal_kind
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/quote_toml_basic_string
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalize_value_for_template(template_value: Option<&String>, raw_value: &str) -> String`

# Calls

- [template_literal_kind](../../../../functions/src/review_ui/config_editor/template_literal_kind.md)
- [quote_toml_basic_string](../../../../functions/src/review_ui/config_editor/quote_toml_basic_string.md)

# Called by

- [save_full_config_values](../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [save_album_config_diff](../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)