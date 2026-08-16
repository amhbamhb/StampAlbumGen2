---
type: Rust Function
title: tail_current_log
resource: src/review_ui.rs#L70-L139
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/get_runtime_log_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tail_current_log(lines_count: usize) -> Vec<String>`

# Calls

- [get_runtime_log_lines](../../../functions/src/review_ui/get_runtime_log_lines.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)