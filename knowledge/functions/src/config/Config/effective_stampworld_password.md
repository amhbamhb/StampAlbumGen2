---
type: Rust Method
title: effective_stampworld_password
resource: src/config.rs#L937-L950
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/account_auth_status_from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_stampworld_password(&self) -> Option<String>`

# Called by

- [account_auth_status_from_config](../../../../functions/src/review_ui/account_auth_status_from_config.md)
- [start_bootstrap_stampworld_job](../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)