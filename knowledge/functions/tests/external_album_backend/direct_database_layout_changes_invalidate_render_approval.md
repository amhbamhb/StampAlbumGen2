---
type: Rust Function
title: direct_database_layout_changes_invalidate_render_approval
resource: tests/external_album_backend.rs#L634-L660
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
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn direct_database_layout_changes_invalidate_render_approval()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)