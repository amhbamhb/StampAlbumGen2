---
type: Rust Function
title: colnect_import_data
resource: src/review_ui.rs#L2858-L3054
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_inbox_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn colnect_import_data( conn: &Connection, _req: ColnectImportRequest, db_path: &str, album_name: &str, ) -> ColnectImportResponse`

# Calls

- [load_album_config](../../../functions/src/review_ui/load_album_config.md)
- [effective_colnect_inbox_path](../../../functions/src/config/Config/effective_colnect_inbox_path.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [album_dir_for_ui](../../../functions/src/review_ui/album_dir_for_ui.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [ingest_colnect_data_with_mode](../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)