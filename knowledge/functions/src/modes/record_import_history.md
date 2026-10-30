---
type: Rust Function
title: record_import_history
resource: src/modes.rs#L221-L264
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn record_import_history( conn: &rusqlite::Connection, country_key: &str, file_path: &std::path::Path, sha256: &str, status: &str, rows_ingested: i64, error_message: Option<&str>, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [metadata](../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)