---
type: Rust Function
title: placement_identities
resource: tests/external_album_backend.rs#L719-L737
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`fn placement_identities(conn: &Connection) -> Vec<(i64, i64, usize)>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [globals_are_editable_at_zero_and_page_placement_identities_survive_saves](../../../functions/tests/external_album_backend/globals_are_editable_at_zero_and_page_placement_identities_survive_saves.md)
- [failed_global_render_persists_draft_and_bad_global_geometry_rolls_back](../../../functions/tests/external_album_backend/failed_global_render_persists_draft_and_bad_global_geometry_rolls_back.md)
- [approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity](../../../functions/tests/external_album_backend/approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [repeated_images_remain_independent_placements_when_one_is_edited_or_deleted](../../../functions/tests/external_album_backend/repeated_images_remain_independent_placements_when_one_is_edited_or_deleted.md)