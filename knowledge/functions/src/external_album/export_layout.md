---
type: Rust Function
title: export_layout
resource: src/external_album.rs#L1392-L1444
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
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
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/read_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_uploaded_assets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/serialize
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate
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
  - target: functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn export_layout(conn: &Connection) -> Result<PathBuf>`

# Calls

- [is_imported](../../../functions/src/external_album/is_imported.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [details](../../../functions/src/external_album/details.md)
- [verify_layout](../../../functions/src/external_album/verify_layout.md)
- [verify_uploaded](../../../functions/src/external_album/verify_uploaded.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [inventory](../../../functions/src/external_album/inventory.md)
- [read_entry](../../../functions/src/external_album/read_entry.md)
- [copy_uploaded_assets](../../../functions/src/external_album/copy_uploaded_assets.md)
- [serialize](../../../functions/src/external_album/parser/serialize.md)

# Called by

- [generate_artifacts](../../../functions/src/external_album/generate_artifacts.md)
- [import_roundtrip_full_copy_and_fidelity_approval_gate](../../../functions/tests/external_album_backend/import_roundtrip_full_copy_and_fidelity_approval_gate.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes](../../../functions/tests/external_album_backend/legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes.md)
- [uploaded_images_are_rendered_exported_and_checksum_gated](../../../functions/tests/external_album_backend/uploaded_images_are_rendered_exported_and_checksum_gated.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)