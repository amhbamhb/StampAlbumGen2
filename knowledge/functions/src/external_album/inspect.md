---
type: Rust Function
title: inspect
resource: src/external_album.rs#L395-L439
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/entry_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn inspect(source: &Path, entry: &Path) -> Result<Inspection>`

# Calls

- [entry_relative](../../../functions/src/external_album/entry_relative.md)
- [err](../../../functions/src/external_album/err.md)
- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [parse](../../../functions/src/external_album/parser/parse.md)
- [asset_errors](../../../functions/src/external_album/asset_errors.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)