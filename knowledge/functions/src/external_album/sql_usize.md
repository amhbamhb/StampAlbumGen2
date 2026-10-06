---
type: Rust Function
title: sql_usize
resource: src/external_album.rs#L83-L86
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sql_usize(row: &rusqlite::Row<'_>, column: usize) -> rusqlite::Result<usize>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [details](../../../functions/src/external_album/details.md)
- [persist_pages](../../../functions/src/external_album/persist_pages.md)