---
type: Rust Function
title: edit_page
resource: src/external_album.rs#L1275-L1374
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
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/reflow/reflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/editable_asset_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/read_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/set_rendered
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
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
  - target: functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn edit_page( root: &Path, name: &str, page: usize, commands: Vec<LayoutCommand>, revision: i64, cli: &str, timeout: Duration, ) -> Result<AlbumDetails>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)
- [details](../../../functions/src/external_album/details.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [relative](../../../functions/src/external_album/relative.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [digest](../../../functions/src/external_album/digest.md)
- [validate_document](../../../functions/src/external_album/parser/validate_document.md)
- [reflow](../../../functions/src/external_album/reflow/reflow.md)
- [validate_page](../../../functions/src/external_album/parser/validate_page.md)
- [editable_asset_errors](../../../functions/src/external_album/editable_asset_errors.md)
- [read_entry](../../../functions/src/external_album/read_entry.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [persist_pages](../../../functions/src/external_album/persist_pages.md)
- [set_rendered](../../../functions/src/external_album/set_rendered.md)

# Called by

- [route](../../../functions/src/review_ui/external_album_api/route.md)
- [page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state](../../../functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [database_write_failure_rolls_back_layout_and_render_artifacts](../../../functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts.md)
- [globals_are_editable_at_zero_and_page_placement_identities_survive_saves](../../../functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves.md)
- [failed_global_render_persists_draft_and_bad_global_geometry_rolls_back](../../../functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity](../../../functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)
- [repeated_images_remain_independent_placements_when_one_is_edited_or_deleted](../../../functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)