---
type: Rust Function
title: render_pair
resource: src/external_album.rs#L938-L1008
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
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_uploaded_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/portable_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/layout_hash
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_pair( album: &Path, conn: &Connection, current: &AlbumDetails, cli: &str, timeout: Duration, ) -> Result<Rendered>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)
- [relative](../../../functions/src/external_album/relative.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [inventory](../../../functions/src/external_album/inventory.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [copy_uploaded_assets](../../../functions/src/external_album/copy_uploaded_assets.md)
- [portable_path](../../../functions/src/external_album/portable_path.md)
- [serialize](../../../functions/src/external_album/parser/serialize.md)
- [asset_errors](../../../functions/src/external_album/asset_errors.md)
- [digest](../../../functions/src/external_album/digest.md)
- [layout_hash](../../../functions/src/external_album/layout_hash.md)

# Called by

- [import_album](../../../functions/src/external_album/import_album.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)