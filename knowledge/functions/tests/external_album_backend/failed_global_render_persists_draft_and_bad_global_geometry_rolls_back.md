---
type: Rust Function
title: failed_global_render_persists_draft_and_bad_global_geometry_rolls_back
resource: tests/external_album_backend.rs#L827-L878
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/external_album_backend/Fixture/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/working_renderer
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
---

# Signature

`fn failed_global_render_persists_draft_and_bad_global_geometry_rolls_back()`

# Calls

- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)