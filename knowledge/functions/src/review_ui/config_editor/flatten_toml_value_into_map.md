---
type: Rust Function
title: flatten_toml_value_into_map
resource: src/review_ui/config_editor.rs#L485-L505
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/config_editor/load_current_config_values
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn flatten_toml_value_into_map( prefix: &str, value: &toml::Value, out: &mut HashMap<String, String>, )`

# Called by

- [load_current_config_values](../../../../functions/src/review_ui/config_editor/load_current_config_values.md)