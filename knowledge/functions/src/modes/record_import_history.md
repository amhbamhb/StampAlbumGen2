---
type: Rust Function
title: record_import_history
resource: src/modes.rs#L225-L268
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

`fn record_import_history( conn: &rusqlite::Connection, country_key: &str, file_path: &std::path::Path, sha256: &str, status: &str, rows_ingested: i64, error_message: Option<&str>, ) -> Result<(), Box<dyn std::error::Error>>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)