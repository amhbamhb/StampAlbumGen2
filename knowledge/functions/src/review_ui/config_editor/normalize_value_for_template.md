---
type: Rust Function
title: normalize_value_for_template
resource: src/review_ui/config_editor.rs#L348-L408
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
  - target: functions/src/review_ui/config_editor/normalizes_windows_paths_and_preserves_valid_toml_strings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path
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
- [normalizes_windows_paths_and_preserves_valid_toml_strings](../../../../functions/src/review_ui/config_editor/normalizes_windows_paths_and_preserves_valid_toml_strings.md)
- [renders_global_and_saves_album_config_with_quoted_windows_path](../../../../functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path.md)