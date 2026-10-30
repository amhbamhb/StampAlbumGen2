---
type: Rust Function
title: save_runtime_string_setting
resource: src/review_ui.rs#L676-L709
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/ensure_runtime_startup_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/find_global_config_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/save_runtime_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/save_runtime_bootstrap_albums_root_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_runtime_string_setting(key: &str, value: Option<&str>) -> Result<(), String>`

# Calls

- [ensure_runtime_startup_files](../../../functions/src/review_ui/ensure_runtime_startup_files.md)
- [find_global_config_path](../../../functions/src/review_ui/find_global_config_path.md)

# Called by

- [save_runtime_albumeasy_cli_path](../../../functions/src/review_ui/save_runtime_albumeasy_cli_path.md)
- [save_runtime_bootstrap_albums_root_path](../../../functions/src/review_ui/save_runtime_bootstrap_albums_root_path.md)