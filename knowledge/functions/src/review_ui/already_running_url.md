---
type: Rust Function
title: already_running_url
resource: src/review_ui.rs#L1691-L1695
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/run_review_ui_bootstrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn already_running_url(err: &dyn std::error::Error) -> Option<String>`

# Called by

- [run_review_ui_bootstrap](../../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)