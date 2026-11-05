---
type: Rust Function
title: brazil_database_roundtrip_and_repeated_placements
resource: tests/external_album_backend.rs#L665-L718
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/working_renderer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
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
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn brazil_database_roundtrip_and_repeated_placements()`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [working_renderer](../../../functions/tests/external_album_backend/Fixture/working_renderer.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [open](../../../functions/src/external_album/open.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [report_brazil_db_counts](../../../functions/tests/external_album_backend/report_brazil_db_counts.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [load_album](../../../functions/src/external_album/load_album.md)