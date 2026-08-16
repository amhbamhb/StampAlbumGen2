---
type: Rust Method
title: finish_with_error
resource: src/progress.rs#L84-L97
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/progress/ProgressIndicator/finish_with_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn finish_with_error(&self, message: &str)`

# Calls

- [finish_with_message](../../../../functions/src/progress/ProgressIndicator/finish_with_message.md)

# Called by

- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)