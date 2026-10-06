---
type: Rust Function
title: asset_errors
resource: src/external_album.rs#L285-L318
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/editable_asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn asset_errors( base: &Path, globals: &[LayoutCommand], pages: &[LayoutPage], ) -> Result<Vec<String>>`

# Calls

- [relative](../../../functions/src/external_album/relative.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [no_links](../../../functions/src/external_album/no_links.md)

# Called by

- [editable_asset_errors](../../../functions/src/external_album/editable_asset_errors.md)
- [inspect](../../../functions/src/external_album/inspect.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)