---
type: Rust Function
title: account_auth_status_from_config
resource: src/review_ui.rs#L753-L774
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/effective_stampworld_username
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_stampworld_password
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_username
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_password
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/get_account_auth_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/site_accounts_preserve_saved_passwords_and_unrelated_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn account_auth_status_from_config(cfg: &crate::config::Config) -> AccountAuthStatusResponse`

# Calls

- [effective_stampworld_username](../../../functions/src/config/Config/effective_stampworld_username.md)
- [effective_stampworld_password](../../../functions/src/config/Config/effective_stampworld_password.md)
- [effective_colnect_username](../../../functions/src/config/Config/effective_colnect_username.md)
- [effective_colnect_password](../../../functions/src/config/Config/effective_colnect_password.md)

# Called by

- [get_account_auth_status](../../../functions/src/review_ui/get_account_auth_status.md)
- [site_accounts_preserve_saved_passwords_and_unrelated_config](../../../functions/src/review_ui/site_accounts_preserve_saved_passwords_and_unrelated_config.md)