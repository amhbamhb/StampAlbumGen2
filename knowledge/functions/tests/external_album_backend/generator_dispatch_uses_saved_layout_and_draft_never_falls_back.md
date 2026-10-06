---
type: Rust Function
title: generator_dispatch_uses_saved_layout_and_draft_never_falls_back
resource: tests/external_album_backend.rs#L528-L563
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
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn generator_dispatch_uses_saved_layout_and_draft_never_falls_back()`

# Calls

- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)