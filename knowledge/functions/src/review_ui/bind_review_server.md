---
type: Rust Function
title: bind_review_server
resource: src/review_ui.rs#L1387-L1468
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/probe_port
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/run_review_ui_bootstrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bind_review_server( bind_addr_opt: Option<&str>, ) -> Result<(Server, String), Box<dyn std::error::Error>>`

# Calls

- [log_startup](../../../functions/src/review_ui/log_startup.md)
- [probe_port](../../../functions/src/review_ui/probe_port.md)

# Called by

- [run_review_ui_bootstrap](../../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)