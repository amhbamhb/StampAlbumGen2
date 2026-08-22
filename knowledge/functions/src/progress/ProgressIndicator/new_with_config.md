---
type: Rust Method
title: new_with_config
resource: src/progress.rs#L19-L43
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
  - target: functions/src/progress/ProgressIndicator/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/new_progress_with_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_with_config(enabled: bool) -> Self`

# Called by

- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [new](../../../../functions/src/progress/ProgressIndicator/new.md)
- [new_progress_with_config](../../../../functions/src/progress/new_progress_with_config.md)