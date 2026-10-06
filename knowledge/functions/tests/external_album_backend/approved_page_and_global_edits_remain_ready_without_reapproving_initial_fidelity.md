---
type: Rust Function
title: approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity
resource: tests/external_album_backend.rs#L1069-L1142
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

`fn approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [placement_identities](../../../functions/tests/external_album_backend/placement_identities.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)