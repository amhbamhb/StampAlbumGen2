---
type: Rust Function
title: editable_asset_errors
resource: src/external_album.rs#L374-L393
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/uploaded_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_errors
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn editable_asset_errors( album: &Path, entry: &Path, globals: &[LayoutCommand], pages: &[LayoutPage], ) -> Result<Vec<String>>`

# Calls

- [uploaded_image](../../../functions/src/external_album/uploaded_image.md)
- [asset_errors](../../../functions/src/external_album/asset_errors.md)

# Called by

- [edit_page](../../../functions/src/external_album/edit_page.md)