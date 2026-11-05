---
type: Rust Function
title: verify_uploaded
resource: src/external_album.rs#L1203-L1218
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
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verify_uploaded(conn: &Connection, album: &Path, current: &AlbumDetails) -> Result<()>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)

# Called by

- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)