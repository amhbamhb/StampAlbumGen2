---
type: Rust Function
title: approval_rejects_modified_pdfs_and_preserved_assets
resource: tests/external_album_backend.rs#L353-L385
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
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn approval_rejects_modified_pdfs_and_preserved_assets()`

# Calls

- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)