---
type: Rust Function
title: try_open_url_once
resource: src/review_ui.rs#L1554-L1579
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
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

- [open](../../../functions/src/external_album/open.md)
- [try_spawn_browser_command_any](../../../functions/src/review_ui/try_spawn_browser_command_any.md)

# Called by

- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)