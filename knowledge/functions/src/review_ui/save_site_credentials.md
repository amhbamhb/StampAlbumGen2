---
type: Rust Function
title: save_site_credentials
resource: src/review_ui.rs#L772-L788
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
  - target: functions/src/review_ui/save_site_credentials_to_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/handle_site_account_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn save_site_credentials( stampworld_user: Option<&str>, stampworld_pass: Option<&str>, colnect_user: Option<&str>, colnect_pass: Option<&str>, ) -> Result<(), String>`

# Calls

- [ensure_runtime_startup_files](../../../functions/src/review_ui/ensure_runtime_startup_files.md)
- [find_global_config_path](../../../functions/src/review_ui/find_global_config_path.md)
- [save_site_credentials_to_path](../../../functions/src/review_ui/save_site_credentials_to_path.md)

# Called by

- [handle_site_account_request](../../../functions/src/review_ui/http_handlers/handle_site_account_request.md)