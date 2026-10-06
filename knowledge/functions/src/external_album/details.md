---
type: Rust Function
title: details
resource: src/external_album.rs#L569-L632
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/sql_usize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn details(conn: &Connection, name: &str) -> Result<AlbumDetails>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)
- [sql_usize](../../../functions/src/external_album/sql_usize.md)
- [validate_document](../../../functions/src/external_album/parser/validate_document.md)

# Called by

- [load_album](../../../functions/src/external_album/load_album.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)