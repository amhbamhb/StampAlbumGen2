---
type: Rust Function
title: cleanup_stampworld_identity_duplicates
resource: src/db_utils/operations.rs#L11-L49
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/operations/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cleanup_stampworld_identity_duplicates( conn: &Connection, keep_rowid: i64, country: &str, stampworld_image_url: &str, ) -> Result<usize>`

# Called by

- [insert_stamp](../../../../functions/src/db_utils/operations/insert_stamp.md)