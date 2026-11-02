---
type: Rust Method
title: effective_colnect_inbox_path
resource: src/config.rs#L1573-L1578
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_colnect_inbox_path(&self) -> Option<&str>`

# Called by

- [main](../../../../functions/src/main.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [colnect_import_data](../../../../functions/src/review_ui/colnect_import_data.md)