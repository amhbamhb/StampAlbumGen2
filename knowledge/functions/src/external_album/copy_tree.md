---
type: Rust Function
title: copy_tree
resource: src/external_album.rs#L441-L469
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/copy_uploaded_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_tree(source: &Path, target: &Path, files: &Inventory) -> Result<()>`

# Calls

- [err](../../../functions/src/external_album/err.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [open](../../../functions/src/external_album/open.md)
- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [copy_uploaded_assets](../../../functions/src/external_album/copy_uploaded_assets.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)