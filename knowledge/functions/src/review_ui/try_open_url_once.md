---
type: Rust Function
title: try_open_url_once
resource: src/review_ui.rs#L1317-L1342
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/try_spawn_browser_command_any
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/open_review_ui_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_open_url_once(url: &str) -> bool`

# Calls

- [try_spawn_browser_command_any](../../../functions/src/review_ui/try_spawn_browser_command_any.md)

# Called by

- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)