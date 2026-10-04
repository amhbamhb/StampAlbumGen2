---
type: Rust Method
title: review_ui_default_sort_mode
resource: src/config.rs#L1443-L1466
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn review_ui_default_sort_mode(&self) -> String`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [main](../../../../functions/src/main.md)