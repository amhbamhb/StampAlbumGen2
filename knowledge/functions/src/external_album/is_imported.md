---
type: Rust Function
title: is_imported
resource: src/external_album.rs#L530-L544
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/imported_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_imported(conn: &Connection) -> Result<bool>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [err](../../../functions/src/external_album/err.md)

# Called by

- [imported_artifacts](../../../functions/src/album_globals/imported_artifacts.md)
- [generate_album_output_conn_with_filters](../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)
- [open](../../../functions/src/external_album/open.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)