---
type: Rust Function
title: find_global_config_path
resource: src/review_ui.rs#L649-L672
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/save_runtime_string_setting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/save_site_credentials
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_global_config_path() -> Option<std::path::PathBuf>`

# Called by

- [save_runtime_string_setting](../../../functions/src/review_ui/save_runtime_string_setting.md)
- [save_site_credentials](../../../functions/src/review_ui/save_site_credentials.md)