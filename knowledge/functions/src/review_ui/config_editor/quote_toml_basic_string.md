---
type: Rust Function
title: quote_toml_basic_string
resource: src/review_ui/config_editor.rs#L344-L347
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

`fn quote_toml_basic_string(input: &str) -> String`

# Called by

- [normalize_value_for_template](../../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)