---
type: Rust Function
title: invalid_edits_roll_back_and_render_failure_saves_retryable_draft
resource: tests/external_album_backend.rs#L448-L526
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
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn invalid_edits_roll_back_and_render_failure_saves_retryable_draft()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [load_album](../../../functions/src/external_album/load_album.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)