---
type: Rust Function
title: is_multiline_array_start
resource: src/review_ui/config_editor.rs#L623-L626
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_multiline_array_start(template_value: &str) -> bool`

# Calls

- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [render_config_from_template_values](../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)