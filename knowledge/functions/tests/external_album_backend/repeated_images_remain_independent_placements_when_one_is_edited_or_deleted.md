---
type: Rust Function
title: repeated_images_remain_independent_placements_when_one_is_edited_or_deleted
resource: tests/external_album_backend.rs#L1486-L1524
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

`fn repeated_images_remain_independent_placements_when_one_is_edited_or_deleted()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)