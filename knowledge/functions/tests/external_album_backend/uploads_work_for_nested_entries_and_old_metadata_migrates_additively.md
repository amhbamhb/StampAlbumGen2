---
type: Rust Function
title: uploads_work_for_nested_entries_and_old_metadata_migrates_additively
resource: tests/external_album_backend.rs#L1418-L1482
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/external_album_backend/Fixture/working_renderer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn uploads_work_for_nested_entries_and_old_metadata_migrates_additively()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [load_album](../../../functions/src/external_album/load_album.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)