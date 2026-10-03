---
type: Rust Function
title: backfill_colnect_links_on_existing_stamps
resource: src/modes.rs#L737-L1095
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/sync_missing_stamps_from_colnect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn backfill_colnect_links_on_existing_stamps( conn: &rusqlite::Connection, ) -> Result<usize, Box<dyn std::error::Error>>`

# Called by

- [sync_missing_stamps_from_colnect](../../../functions/src/modes/sync_missing_stamps_from_colnect.md)