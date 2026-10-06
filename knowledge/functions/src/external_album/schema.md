---
type: Rust Function
title: schema
resource: src/external_album.rs#L471-L528
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema(conn: &Connection) -> Result<()>`

# Calls

- [err](../../../functions/src/external_album/err.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [open](../../../functions/src/external_album/open.md)
- [import_album](../../../functions/src/external_album/import_album.md)