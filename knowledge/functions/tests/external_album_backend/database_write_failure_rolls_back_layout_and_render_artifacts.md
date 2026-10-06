---
type: Rust Function
title: database_write_failure_rolls_back_layout_and_render_artifacts
resource: tests/external_album_backend.rs#L588-L628
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
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn database_write_failure_rolls_back_layout_and_render_artifacts()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [load_album](../../../functions/src/external_album/load_album.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)