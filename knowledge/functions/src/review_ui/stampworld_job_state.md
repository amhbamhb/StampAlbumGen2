---
type: Rust Function
title: stampworld_job_state
resource: src/review_ui.rs#L46-L48
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/set_stampworld_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stampworld_job_state() -> &'static Mutex<StampWorldJobState>`

# Called by

- [set_stampworld_state](../../../functions/src/review_ui/set_stampworld_state.md)
- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)