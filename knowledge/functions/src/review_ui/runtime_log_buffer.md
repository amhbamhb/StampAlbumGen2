---
type: Rust Function
title: runtime_log_buffer
resource: src/review_ui.rs#L43-L45
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/get_runtime_log_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn runtime_log_buffer() -> &'static Mutex<VecDeque<String>>`

# Called by

- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [get_runtime_log_lines](../../../functions/src/review_ui/get_runtime_log_lines.md)