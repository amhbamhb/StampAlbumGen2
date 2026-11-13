---
type: Rust Function
title: uploaded_images_are_rendered_exported_and_checksum_gated
resource: tests/external_album_backend.rs#L1331-L1437
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/external_album_backend/Fixture/working_renderer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/placement_identities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn uploaded_images_are_rendered_exported_and_checksum_gated()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [load_album](../../../functions/src/external_album/load_album.md)