---
type: Rust Function
title: selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected
resource: tests/external_album_backend.rs#L1566-L1624
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/external_album_backend/Fixture/working_renderer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected()`

# Calls

- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [open](../../../functions/src/external_album/open.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)