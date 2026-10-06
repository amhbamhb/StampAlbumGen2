---
type: Rust Function
title: save_runtime_bootstrap_albums_root_path
resource: src/review_ui.rs#L869-L871
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/save_runtime_string_setting
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_runtime_bootstrap_albums_root_path(path: Option<&str>) -> Result<(), String>`

# Calls

- [save_runtime_string_setting](../../../functions/src/review_ui/save_runtime_string_setting.md)

# Called by

- [set_bootstrap_albums_root_override](../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)