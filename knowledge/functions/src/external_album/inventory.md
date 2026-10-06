---
type: Rust Function
title: inventory
resource: src/external_album.rs#L201-L203
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/scan_inventory
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

`fn inventory(source: &Path) -> Result<Inventory>`

# Calls

- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)

# Called by

- [copy_uploaded_assets](../../../functions/src/external_album/copy_uploaded_assets.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)