---
type: Rust Function
title: generated_artifacts_from_path
resource: src/album_globals/mod.rs#L238-L253
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/albumeasy_auto_pdf_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generated_artifacts_from_path( output_txt_path: PathBuf, generate_pdf_override: Option<bool>, ) -> GeneratedAlbumArtifacts`

# Calls

- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [effective_albumeasy_cli_path](../../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [albumeasy_auto_pdf_enabled](../../../functions/src/config/Config/albumeasy_auto_pdf_enabled.md)
- [maybe_generate_album_pdf_with_settings](../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)

# Called by

- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [generate_album_output](../../../functions/src/album_globals/generate_album_output.md)