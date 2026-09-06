---
type: Rust Function
title: has_import_checksum
resource: src/modes.rs#L209-L219
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

`fn has_import_checksum( conn: &rusqlite::Connection, country_key: &str, sha256: &str, ) -> Result<bool, Box<dyn std::error::Error>>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)