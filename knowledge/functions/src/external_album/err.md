---
type: Rust Function
title: err
resource: src/external_album.rs#L71-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/sql_integer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/set_rendered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_uploaded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/read_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn err(context: &str, error: impl std::fmt::Display) -> String`

# Called by

- [json](../../../functions/src/external_album/json.md)
- [sql_integer](../../../functions/src/external_album/sql_integer.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [digest](../../../functions/src/external_album/digest.md)
- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [inspect](../../../functions/src/external_album/inspect.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [schema](../../../functions/src/external_album/schema.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [paths](../../../functions/src/external_album/paths.md)
- [open](../../../functions/src/external_album/open.md)
- [details](../../../functions/src/external_album/details.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)
- [persist_pages](../../../functions/src/external_album/persist_pages.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [render](../../../functions/src/external_album/render.md)
- [verify_layout](../../../functions/src/external_album/verify_layout.md)
- [set_rendered](../../../functions/src/external_album/set_rendered.md)
- [verify_uploaded](../../../functions/src/external_album/verify_uploaded.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [read_entry](../../../functions/src/external_album/read_entry.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)