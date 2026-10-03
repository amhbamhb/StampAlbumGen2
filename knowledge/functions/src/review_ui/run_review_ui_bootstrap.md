---
type: Rust Function
title: run_review_ui_bootstrap
resource: src/review_ui.rs#L2943-L2999
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/bind_review_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/already_running_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_review_ui_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_review_ui_bootstrap( bind_addr_opt: Option<&str>, default_inbox_path: Option<&str>, open_browser: bool, ) -> Result<ReviewUiBootstrapOutcome, Box<dyn std::error::Error>>`

# Calls

- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [bind_review_server](../../../functions/src/review_ui/bind_review_server.md)
- [already_running_url](../../../functions/src/review_ui/already_running_url.md)
- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)

# Called by

- [main](../../../functions/src/main.md)