---
type: Rust Function
title: copy_uploaded_assets
resource: src/external_album.rs#L358-L372
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/uploaded_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inventory
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_uploaded_assets(album: &Path, entry_base: &Path, pages: &[LayoutPage]) -> Result<()>`

# Calls

- [uploaded_image](../../../functions/src/external_album/uploaded_image.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [inventory](../../../functions/src/external_album/inventory.md)

# Called by

- [render_pair](../../../functions/src/external_album/render_pair.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)