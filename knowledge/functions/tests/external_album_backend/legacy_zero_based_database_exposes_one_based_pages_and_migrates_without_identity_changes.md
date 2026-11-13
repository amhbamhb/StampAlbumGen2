---
type: Rust Function
title: legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes
resource: tests/external_album_backend.rs#L1169-L1254
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
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/placement_identities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [load_album](../../../functions/src/external_album/load_album.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)