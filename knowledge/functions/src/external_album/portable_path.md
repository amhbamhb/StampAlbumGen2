---
type: Rust Function
title: portable_path
resource: src/external_album.rs#L148-L153
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/entry_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn portable_path(path: &Path) -> Result<String>`

# Called by

- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [entry_relative](../../../functions/src/external_album/entry_relative.md)
- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)