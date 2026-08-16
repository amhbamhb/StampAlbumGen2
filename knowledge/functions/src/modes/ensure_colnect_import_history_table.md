---
type: Rust Function
title: ensure_colnect_import_history_table
resource: src/modes.rs#L185-L211
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ensure_colnect_import_history_table( conn: &rusqlite::Connection, ) -> Result<(), Box<dyn std::error::Error>>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)