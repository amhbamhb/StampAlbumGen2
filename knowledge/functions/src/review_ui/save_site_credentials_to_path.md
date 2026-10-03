---
type: Rust Function
title: save_site_credentials_to_path
resource: src/review_ui.rs#L790-L853
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/save_site_credentials
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/site_accounts_preserve_saved_passwords_and_unrelated_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_site_credentials_to_path( config_path: &Path, stampworld_user: Option<&str>, stampworld_pass: Option<&str>, colnect_user: Option<&str>, colnect_pass: Option<&str>, ) -> Result<(), String>`

# Called by

- [save_site_credentials](../../../functions/src/review_ui/save_site_credentials.md)
- [site_accounts_preserve_saved_passwords_and_unrelated_config](../../../functions/src/review_ui/site_accounts_preserve_saved_passwords_and_unrelated_config.md)