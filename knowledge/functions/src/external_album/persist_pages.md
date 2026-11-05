---
type: Rust Function
title: persist_pages
resource: src/external_album.rs#L708-L806
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/sql_integer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/sql_usize
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn persist_pages(conn: &Connection, pages: &[LayoutPage], selected: Option<usize>) -> Result<()>`

# Calls

- [sql_integer](../../../functions/src/external_album/sql_integer.md)
- [err](../../../functions/src/external_album/err.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [sql_usize](../../../functions/src/external_album/sql_usize.md)

# Called by

- [import_album](../../../functions/src/external_album/import_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)