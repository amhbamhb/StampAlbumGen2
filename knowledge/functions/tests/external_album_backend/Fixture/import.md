---
type: Rust Method
title: import
resource: tests/external_album_backend.rs#L71-L81
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/failed_draft_can_retry_but_copied_pdf_cannot_validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/approval_rejects_modified_pdfs_and_preserved_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn import(&self, cli: &Path) -> AlbumDetails`

# Calls

- [import_album](../../../../functions/src/external_album/import_album.md)

# Called by

- [import_roundtrip_full_copy_and_fidelity_approval_gate](../../../../functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate.md)
- [failed_draft_can_retry_but_copied_pdf_cannot_validate](../../../../functions/tests/external_album_backend/failed_draft_can_retry_but_copied_pdf_cannot_validate.md)
- [approval_rejects_modified_pdfs_and_preserved_assets](../../../../functions/tests/external_album_backend/approval_rejects_modified_pdfs_and_preserved_assets.md)
- [page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state](../../../../functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [generator_dispatch_uses_saved_layout_and_draft_never_falls_back](../../../../functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back.md)
- [database_write_failure_rolls_back_layout_and_render_artifacts](../../../../functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts.md)
- [direct_database_layout_changes_invalidate_render_approval](../../../../functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval.md)
- [globals_are_editable_at_zero_and_page_placement_identities_survive_saves](../../../../functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves.md)
- [failed_global_render_persists_draft_and_bad_global_geometry_rolls_back](../../../../functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back.md)
- [approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity](../../../../functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [repeated_images_remain_independent_placements_when_one_is_edited_or_deleted](../../../../functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted.md)