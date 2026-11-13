---
type: Rust Function
title: renders_global_and_saves_album_config_with_quoted_windows_path
resource: src/review_ui/config_editor.rs#L991-L1038
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/normalize_value_for_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn renders_global_and_saves_album_config_with_quoted_windows_path()`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [normalize_value_for_template](../../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)
- [render_config_from_template_values](../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)