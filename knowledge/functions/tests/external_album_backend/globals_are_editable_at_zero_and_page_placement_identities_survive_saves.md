---
type: Rust Function
title: globals_are_editable_at_zero_and_page_placement_identities_survive_saves
resource: tests/external_album_backend.rs#L741-L821
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

`fn globals_are_editable_at_zero_and_page_placement_identities_survive_saves()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)