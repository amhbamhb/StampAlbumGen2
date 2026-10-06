---
type: Rust Function
title: windows_reserved
resource: src/external_album.rs#L123-L129
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/external_album/validate_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn windows_reserved(name: &str) -> bool`

# Called by

- [validate_slug](../../../functions/src/external_album/validate_slug.md)
- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)