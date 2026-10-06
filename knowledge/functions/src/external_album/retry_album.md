---
type: Rust Function
title: retry_album
resource: src/external_album.rs#L1204-L1232
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/set_rendered
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/failed_draft_can_retry_but_copied_pdf_cannot_validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/approval_rejects_modified_pdfs_and_preserved_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn retry_album(root: &Path, name: &str, cli: &str, timeout: Duration) -> Result<AlbumDetails>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)
- [details](../../../functions/src/external_album/details.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [set_rendered](../../../functions/src/external_album/set_rendered.md)

# Called by

- [route](../../../functions/src/review_ui/external_album_api/route.md)
- [failed_draft_can_retry_but_copied_pdf_cannot_validate](../../../functions/tests/external_album_backend/failed_draft_can_retry_but_copied_pdf_cannot_validate.md)
- [approval_rejects_modified_pdfs_and_preserved_assets](../../../functions/tests/external_album_backend/approval_rejects_modified_pdfs_and_preserved_assets.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [direct_database_layout_changes_invalidate_render_approval](../../../functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)