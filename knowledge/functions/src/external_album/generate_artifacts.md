---
type: Rust Function
title: generate_artifacts
resource: src/external_album.rs#L1446-L1474
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/albumeasy_auto_pdf_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/albumeasy_pdf_timeout_secs
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/imported_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_filters
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn generate_artifacts( conn: &Connection, pdf_override: Option<bool>, ) -> Result<crate::album_globals::GeneratedAlbumArtifacts>`

# Calls

- [export_layout](../../../functions/src/external_album/export_layout.md)
- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [albumeasy_auto_pdf_enabled](../../../functions/src/config/Config/albumeasy_auto_pdf_enabled.md)
- [effective_albumeasy_cli_path](../../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [albumeasy_pdf_timeout_secs](../../../functions/src/config/Config/albumeasy_pdf_timeout_secs.md)

# Called by

- [imported_artifacts](../../../functions/src/album_globals/imported_artifacts.md)
- [generate_album_output_conn_with_filters](../../../functions/src/album_globals/generate_album_output_conn_with_filters.md)