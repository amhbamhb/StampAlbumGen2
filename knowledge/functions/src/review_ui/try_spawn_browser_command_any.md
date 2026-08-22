---
type: Rust Function
title: try_spawn_browser_command_any
resource: src/review_ui.rs#L1308-L1315
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/try_spawn_browser_command
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/try_open_url_once
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_spawn_browser_command_any(programs: &[&str], args: &[&str]) -> bool`

# Calls

- [try_spawn_browser_command](../../../functions/src/review_ui/try_spawn_browser_command.md)

# Called by

- [try_open_url_once](../../../functions/src/review_ui/try_open_url_once.md)