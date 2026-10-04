---
type: Rust Function
title: run_colnect_inbox_mode
resource: src/modes.rs#L305-L567
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/progress/ProgressIndicator/new_with_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/set_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/collect_colnect_csv_files_rec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/country_slug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/parse_colnect_csv_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/move_to_country_inbox
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/infer_country_from_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/ensure_colnect_import_history_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/file_sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/has_import_checksum
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/record_import_history
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
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run_colnect_inbox_mode( cli: &crate::cli::Cli, show_progress: bool, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [new_with_config](../../../functions/src/progress/ProgressIndicator/new_with_config.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [set_message](../../../functions/src/progress/ProgressIndicator/set_message.md)
- [collect_colnect_csv_files_rec](../../../functions/src/modes/collect_colnect_csv_files_rec.md)
- [country_slug](../../../functions/src/modes/country_slug.md)
- [parse_colnect_csv_filename](../../../functions/src/modes/parse_colnect_csv_filename.md)
- [move_to_country_inbox](../../../functions/src/modes/move_to_country_inbox.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [infer_country_from_stamps](../../../functions/src/modes/infer_country_from_stamps.md)
- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)
- [ensure_colnect_import_history_table](../../../functions/src/modes/ensure_colnect_import_history_table.md)
- [file_sha256](../../../functions/src/modes/file_sha256.md)
- [has_import_checksum](../../../functions/src/modes/has_import_checksum.md)
- [effective_default_catalog_code](../../../functions/src/config/Config/effective_default_catalog_code.md)
- [ingest_colnect_data_with_mode](../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)
- [record_import_history](../../../functions/src/modes/record_import_history.md)
- [with_album_config_override_async](../../../functions/src/config/with_album_config_override_async.md)
- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [finish_with_message](../../../functions/src/progress/ProgressIndicator/finish_with_message.md)

# Called by

- [run_database_mode](../../../functions/src/modes/run_database_mode.md)