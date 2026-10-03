---
type: Rust Function
title: open_review_ui_url
resource: src/review_ui.rs#L1565-L1584
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/try_open_url_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
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

`fn open_review_ui_url(url: &str)`

# Calls

- [log_startup](../../../functions/src/review_ui/log_startup.md)
- [try_open_url_once](../../../functions/src/review_ui/try_open_url_once.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [run_review_ui_bootstrap](../../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)