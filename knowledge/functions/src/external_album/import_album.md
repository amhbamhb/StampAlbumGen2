---
type: Rust Function
title: import_album
resource: src/external_album.rs#L808-L936
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/entry_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
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
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
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

`pub fn import_album( root: &Path, name: &str, source: &Path, entry: &Path, cli: &str, timeout: Duration, ) -> Result<AlbumDetails>`

# Calls

- [entry_relative](../../../functions/src/external_album/entry_relative.md)
- [err](../../../functions/src/external_album/err.md)
- [slug](../../../functions/src/external_album/slug.md)
- [inventory](../../../functions/src/external_album/inventory.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [schema](../../../functions/src/external_album/schema.md)
- [persist_pages](../../../functions/src/external_album/persist_pages.md)
- [details](../../../functions/src/external_album/details.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [set_rendered](../../../functions/src/external_album/set_rendered.md)
- [load_album](../../../functions/src/external_album/load_album.md)

# Called by

- [route](../../../functions/src/review_ui/external_album_api/route.md)
- [import](../../../functions/tests/external_album_backend/Fixture/import.md)
- [renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation](../../../functions/tests/external_album_backend/renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [uploads_work_for_nested_entries_and_old_metadata_migrates_additively](../../../functions/tests/external_album_backend/uploads_work_for_nested_entries_and_old_metadata_migrates_additively.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)