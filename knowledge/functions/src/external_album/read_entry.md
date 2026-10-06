---
type: Rust Function
title: read_entry
resource: src/external_album.rs#L1376-L1383
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
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_entry(conn: &Connection) -> Result<PathBuf>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)
- [relative](../../../functions/src/external_album/relative.md)

# Called by

- [edit_page](../../../functions/src/external_album/edit_page.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)