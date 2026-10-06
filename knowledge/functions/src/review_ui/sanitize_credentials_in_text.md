---
type: Rust Function
title: sanitize_credentials_in_text
resource: src/review_ui.rs#L55-L68
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sanitize_credentials_in_text(raw: &str) -> String`

# Called by

- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)