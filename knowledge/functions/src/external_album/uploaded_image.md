---
type: Rust Function
title: uploaded_image
resource: src/external_album.rs#L320-L334
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/uploaded_hash
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_uploaded_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/editable_asset_errors
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn uploaded_image(command: &LayoutCommand) -> Result<Option<PathBuf>>`

# Calls

- [relative](../../../functions/src/external_album/relative.md)

# Called by

- [uploaded_hash](../../../functions/src/external_album/uploaded_hash.md)
- [copy_uploaded_assets](../../../functions/src/external_album/copy_uploaded_assets.md)
- [editable_asset_errors](../../../functions/src/external_album/editable_asset_errors.md)