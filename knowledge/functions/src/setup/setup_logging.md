---
type: Rust Function
title: setup_logging
resource: src/setup.rs#L71-L106
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn setup_logging() -> Result<(), SetupError>`

# Calls

- [formatted_now](../../../functions/src/time_utils/formatted_now.md)

# Called by

- [main](../../../functions/src/main.md)