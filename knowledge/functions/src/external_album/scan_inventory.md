---
type: Rust Function
title: scan_inventory
resource: src/external_album.rs#L205-L264
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/portable_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/windows_reserved
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scan_inventory(source: &Path, checksum: bool) -> Result<Inventory>`

# Calls

- [err](../../../functions/src/external_album/err.md)
- [portable_path](../../../functions/src/external_album/portable_path.md)
- [windows_reserved](../../../functions/src/external_album/windows_reserved.md)
- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [inventory](../../../functions/src/external_album/inventory.md)
- [inspect](../../../functions/src/external_album/inspect.md)