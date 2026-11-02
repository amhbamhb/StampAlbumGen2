---
type: Rust Function
title: init_db
resource: crates/stamp_album_db/src/lib.rs#L592-L635
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn init_db(db_path: &str) -> Result<Connection>`

# Calls

- [open](../../../../functions/src/external_album/open.md)