---
type: Rust Function
title: ensure_runtime_startup_files
resource: src/review_ui.rs#L698-L705
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/save_runtime_string_setting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/save_site_credentials
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ensure_runtime_startup_files() -> Result<(), String>`

# Calls

- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)

# Called by

- [save_runtime_string_setting](../../../functions/src/review_ui/save_runtime_string_setting.md)
- [save_site_credentials](../../../functions/src/review_ui/save_site_credentials.md)
- [set_bootstrap_albums_root_override](../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)