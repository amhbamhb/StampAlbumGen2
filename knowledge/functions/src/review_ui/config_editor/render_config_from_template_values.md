---
type: Rust Function
title: render_config_from_template_values
resource: src/review_ui/config_editor.rs#L619-L735
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/parse_config_key_value_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/config_field_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/is_multiline_array_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_config_from_template_values( template_content: &str, values: &HashMap<String, String>, ) -> String`

# Calls

- [parse_config_key_value_line](../../../../functions/src/review_ui/config_editor/parse_config_key_value_line.md)
- [config_field_id](../../../../functions/src/review_ui/config_editor/config_field_id.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [is_multiline_array_start](../../../../functions/src/review_ui/config_editor/is_multiline_array_start.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [save_full_config_values](../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [renders_global_and_saves_album_config_with_quoted_windows_path](../../../../functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path.md)