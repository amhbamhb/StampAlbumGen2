---
type: Rust Function
title: site_accounts_preserve_saved_passwords_and_unrelated_config
resource: src/review_ui.rs#L3247-L3298
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/save_site_credentials_to_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/account_auth_status_from_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn site_accounts_preserve_saved_passwords_and_unrelated_config()`

# Calls

- [save_site_credentials_to_path](../../../functions/src/review_ui/save_site_credentials_to_path.md)
- [account_auth_status_from_config](../../../functions/src/review_ui/account_auth_status_from_config.md)