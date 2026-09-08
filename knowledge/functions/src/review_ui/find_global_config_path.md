---
type: Rust Function
title: find_global_config_path
resource: src/review_ui.rs#L575-L598
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/save_runtime_string_setting
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_global_config_path() -> Option<std::path::PathBuf>`

# Called by

- [save_runtime_string_setting](../../../functions/src/review_ui/save_runtime_string_setting.md)