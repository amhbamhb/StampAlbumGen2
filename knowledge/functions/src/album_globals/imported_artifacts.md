---
type: Rust Function
title: imported_artifacts
resource: src/album_globals/mod.rs#L293-L310
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_prepared_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn imported_artifacts( conn: &Connection, generate_pdf_override: Option<bool>, ) -> Option<GeneratedAlbumArtifacts>`

# Calls

- [is_imported](../../../functions/src/external_album/is_imported.md)
- [generate_artifacts](../../../functions/src/external_album/generate_artifacts.md)

# Called by

- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [generate_album_output_conn_with_prepared_pdf](../../../functions/src/album_globals/generate_album_output_conn_with_prepared_pdf.md)
- [generate_album_output](../../../functions/src/album_globals/generate_album_output.md)