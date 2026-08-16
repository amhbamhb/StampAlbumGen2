---
type: Rust Function
title: log_startup
resource: src/review_ui.rs#L1485-L1515
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/open_review_ui_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/bind_review_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/probe_port
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn log_startup(msg: &str)`

# Calls

- [formatted_now](../../../functions/src/time_utils/formatted_now.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)
- [bind_review_server](../../../functions/src/review_ui/bind_review_server.md)
- [probe_port](../../../functions/src/review_ui/probe_port.md)