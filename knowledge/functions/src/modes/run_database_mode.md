---
type: Rust Function
title: run_database_mode
resource: src/modes.rs#L1231-L1447
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/new_with_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/set_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/parse_colnect_csv_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/finish_with_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/has_downloaded_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/with_album_config_override_async
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/finish_with_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn run_database_mode( cli: &crate::cli::Cli, show_progress: bool, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [new_with_config](../../../functions/src/progress/ProgressIndicator/new_with_config.md)
- [set_message](../../../functions/src/progress/ProgressIndicator/set_message.md)
- [parse_colnect_csv_filename](../../../functions/src/modes/parse_colnect_csv_filename.md)
- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)
- [finish_with_error](../../../functions/src/progress/ProgressIndicator/finish_with_error.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [has_downloaded_images](../../../functions/src/modes/has_downloaded_images.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [effective_default_catalog_code](../../../functions/src/config/Config/effective_default_catalog_code.md)
- [ingest_colnect_data_with_default_catalog](../../../functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog.md)
- [with_album_config_override_async](../../../functions/src/config/with_album_config_override_async.md)
- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [finish_with_message](../../../functions/src/progress/ProgressIndicator/finish_with_message.md)

# Called by

- [main](../../../functions/src/main.md)