---
type: Rust Function
title: no_links
resource: src/external_album.rs#L155-L173
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn no_links(path: &Path) -> Result<()>`

# Calls

- [err](../../../functions/src/external_album/err.md)

# Called by

- [asset_errors](../../../functions/src/external_album/asset_errors.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [paths](../../../functions/src/external_album/paths.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [render](../../../functions/src/external_album/render.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)