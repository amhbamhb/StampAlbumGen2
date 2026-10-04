---
type: Rust Function
title: get_runtime_log_lines
resource: src/review_ui.rs#L78-L83
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/runtime_log_buffer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_runtime_log_lines() -> Vec<String>`

# Calls

- [runtime_log_buffer](../../../functions/src/review_ui/runtime_log_buffer.md)

# Called by

- [tail_current_log](../../../functions/src/review_ui/tail_current_log.md)