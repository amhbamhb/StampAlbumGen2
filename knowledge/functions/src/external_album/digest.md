---
type: Rust Function
title: digest
resource: src/external_album.rs#L181-L199
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/layout_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn digest(path: &Path) -> Result<String>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)

# Called by

- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [layout_hash](../../../functions/src/external_album/layout_hash.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)