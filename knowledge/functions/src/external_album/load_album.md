---
type: Rust Function
title: load_album
resource: src/external_album.rs#L634-L637
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/pdf_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
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
---

# Signature

`pub fn load_album(root: &Path, name: &str) -> Result<AlbumDetails>`

# Calls

- [open](../../../functions/src/external_album/open.md)
- [details](../../../functions/src/external_album/details.md)

# Called by

- [import_album](../../../functions/src/external_album/import_album.md)
- [route](../../../functions/src/review_ui/external_album_api/route.md)
- [pdf_path](../../../functions/src/review_ui/external_album_api/pdf_path.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [database_write_failure_rolls_back_layout_and_render_artifacts](../../../functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)