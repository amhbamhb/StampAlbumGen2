---
type: Rust Function
title: failed_draft_can_retry_but_copied_pdf_cannot_validate
resource: tests/external_album_backend.rs#L300-L319
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

`fn failed_draft_can_retry_but_copied_pdf_cannot_validate()`

# Calls

- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)