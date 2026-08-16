---
type: Rust Method
title: finish_with_message
resource: src/progress.rs#L68-L81
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
  - target: functions/src/progress/ProgressIndicator/finish_with_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn finish_with_message(&self, message: &str)`

# Called by

- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [finish_with_error](../../../../functions/src/progress/ProgressIndicator/finish_with_error.md)