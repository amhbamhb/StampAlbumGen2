---
type: Rust Function
title: brazil_real_albumeasy_import_approval_and_page_edit
resource: tests/external_album_backend.rs#L882-L1046
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/source_checksums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/rasterize_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/report_brazil_db_counts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn brazil_real_albumeasy_import_approval_and_page_edit()`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [source_checksums](../../../functions/tests/external_album_backend/source_checksums.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [rasterize_pdf](../../../functions/tests/external_album_backend/rasterize_pdf.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [open](../../../functions/src/external_album/open.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [report_brazil_db_counts](../../../functions/tests/external_album_backend/report_brazil_db_counts.md)