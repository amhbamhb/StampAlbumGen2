---
type: Rust Function
title: parse_colnect_csv_filename
resource: src/modes.rs#L5-L41
visibility: private
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
---

# Signature

`fn parse_colnect_csv_filename(csv_path: &str) -> Option<(String, String)>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)