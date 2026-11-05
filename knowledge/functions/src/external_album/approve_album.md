---
type: Rust Function
title: approve_album
resource: src/external_album.rs#L1248-L1287
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
  - target: functions/src/external_album/verify_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_uploaded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
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
  called_by:
  - target: functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
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
  - target: functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn approve_album(root: &Path, name: &str, revision: i64) -> Result<AlbumDetails>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)
- [details](../../../functions/src/external_album/details.md)
- [verify_layout](../../../functions/src/external_album/verify_layout.md)
- [verify_uploaded](../../../functions/src/external_album/verify_uploaded.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [relative](../../../functions/src/external_album/relative.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [import_roundtrip_full_copy_and_fidelity_approval_gate](../../../functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate.md)
- [page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state](../../../functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state.md)
- [generator_dispatch_uses_saved_layout_and_draft_never_falls_back](../../../functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back.md)
- [direct_database_layout_changes_invalidate_render_approval](../../../functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity](../../../functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)