---
type: Rust Module
title: external_album_backend
resource: tests/external_album_backend.rs#L1-L1601
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-external-album
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-permissionsext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-symlink
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Fixture](../../classes/tests/external_album_backend/Fixture.md)
- [new](../../functions/tests/external_album_backend/Fixture/new.md)
- [renderer](../../functions/tests/external_album_backend/Fixture/renderer.md)
- [working_renderer](../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import](../../functions/tests/external_album_backend/Fixture/import.md)
- [db](../../functions/tests/external_album_backend/Fixture/db.md)
- [inspection_preserves_counts_legacy_units_and_source](../../functions/tests/external_album_backend/inspection_preserves_counts_legacy_units_and_source.md)
- [compatibility_and_missing_images_are_explicit](../../functions/tests/external_album_backend/compatibility_and_missing_images_are_explicit.md)
- [rejects_malformed_commands_and_unsafe_cross_platform_paths](../../functions/tests/external_album_backend/rejects_malformed_commands_and_unsafe_cross_platform_paths.md)
- [rejects_case_collisions_reserved_names_and_nested_destination_without_writes](../../functions/tests/external_album_backend/rejects_case_collisions_reserved_names_and_nested_destination_without_writes.md)
- [rejects_symlinks_and_special_files](../../functions/tests/external_album_backend/rejects_symlinks_and_special_files.md)
- [import_roundtrip_full_copy_and_fidelity_approval_gate](../../functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate.md)
- [failed_draft_can_retry_but_copied_pdf_cannot_validate](../../functions/tests/external_album_backend/failed_draft_can_retry_but_copied_pdf_cannot_validate.md)
- [renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation](../../functions/tests/external_album_backend/renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation.md)
- [approval_rejects_modified_pdfs_and_preserved_assets](../../functions/tests/external_album_backend/approval_rejects_modified_pdfs_and_preserved_assets.md)
- [page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state](../../functions/tests/external_album_backend/page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [generator_dispatch_uses_saved_layout_and_draft_never_falls_back](../../functions/tests/external_album_backend/generator_dispatch_uses_saved_layout_and_draft_never_falls_back.md)
- [brazil_exact_acceptance_counts](../../functions/tests/external_album_backend/brazil_exact_acceptance_counts.md)
- [database_write_failure_rolls_back_layout_and_render_artifacts](../../functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts.md)
- [direct_database_layout_changes_invalidate_render_approval](../../functions/tests/external_album_backend/direct_database_layout_changes_invalidate_render_approval.md)
- [brazil_database_roundtrip_and_repeated_placements](../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [placement_identities](../../functions/tests/external_album_backend/placement_identities.md)
- [globals_are_editable_at_zero_and_page_placement_identities_survive_saves](../../functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves.md)
- [failed_global_render_persists_draft_and_bad_global_geometry_rolls_back](../../functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [rasterize_pdf](../../functions/tests/external_album_backend/rasterize_pdf.md)
- [approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity](../../functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [report_brazil_db_counts](../../functions/tests/external_album_backend/report_brazil_db_counts.md)
- [source_checksums](../../functions/tests/external_album_backend/source_checksums.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)
- [repeated_images_remain_independent_placements_when_one_is_edited_or_deleted](../../functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted.md)
- [inspection_does_not_read_or_decode_image_contents](../../functions/tests/external_album_backend/inspection_does_not_read_or_decode_image_contents.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)

# Imports

- `rusqlite::Connection`
- `stamp_album_gen2::external_album::*`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `std::time::{Duration, Instant}`
- `std::os::unix::fs::PermissionsExt`
- `std::os::unix::fs::symlink`
- `sha2::{Digest, Sha256}`
- `std::io::Read`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)