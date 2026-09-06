---
type: Rust Function
title: template_literal_kind
resource: src/review_ui/config_editor.rs#L324-L342
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/config_editor/normalize_value_for_template
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn template_literal_kind(template_value: &str) -> &'static str`

# Called by

- [normalize_value_for_template](../../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)