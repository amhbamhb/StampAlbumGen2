---
type: Rust Function
title: relative
resource: src/external_album.rs#L131-L146
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/entry_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/uploaded_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/read_entry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn relative(value: &str) -> Result<PathBuf>`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [entry_relative](../../../functions/src/external_album/entry_relative.md)
- [asset_errors](../../../functions/src/external_album/asset_errors.md)
- [uploaded_image](../../../functions/src/external_album/uploaded_image.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [read_entry](../../../functions/src/external_album/read_entry.md)