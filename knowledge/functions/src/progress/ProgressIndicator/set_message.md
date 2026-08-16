---
type: Rust Method
title: set_message
resource: src/progress.rs#L46-L60
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/set_message_format
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_message(&self, message: &str)`

# Called by

- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [set_message_format](../../../../functions/src/progress/ProgressIndicator/set_message_format.md)