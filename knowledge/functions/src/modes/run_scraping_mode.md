---
type: Rust Function
title: run_scraping_mode
resource: src/modes.rs#L1427-L1894
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/progress/ProgressIndicator/new_with_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/finish_with_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/set_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_inbox_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/set_db_path_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/wikipedia_ingest_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/ingest/ingest_wikipedia_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_wikipedia_ingest_max_entries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_wikipedia_ingest_series_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/new_for_api
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/fetch_and_cache_content
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
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/with_album_config_override
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

`pub fn run_scraping_mode( cli: &crate::cli::Cli, show_progress: bool, source: &str, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [new_with_config](../../../functions/src/progress/ProgressIndicator/new_with_config.md)
- [finish_with_error](../../../functions/src/progress/ProgressIndicator/finish_with_error.md)
- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)
- [set_message](../../../functions/src/progress/ProgressIndicator/set_message.md)
- [load_slug_overrides](../../../functions/src/config/load_slug_overrides.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [load_config](../../../functions/src/config/load_config.md)
- [effective_colnect_inbox_path](../../../functions/src/config/Config/effective_colnect_inbox_path.md)
- [set_db_path_from_cli](../../../functions/src/file_utils/set_db_path_from_cli.md)
- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [process_1847us_page](../../../functions/src/us1847/collector/process_1847us_page.md)
- [wikipedia_ingest_enabled](../../../functions/src/config/Config/wikipedia_ingest_enabled.md)
- [ingest_wikipedia_to_db](../../../functions/src/wikipedia/ingest/ingest_wikipedia_to_db.md)
- [effective_wikipedia_ingest_max_entries](../../../functions/src/config/Config/effective_wikipedia_ingest_max_entries.md)
- [effective_wikipedia_ingest_series_prefix](../../../functions/src/config/Config/effective_wikipedia_ingest_series_prefix.md)
- [copy_flag_for_album](../../../functions/src/flagdb/copy_flag_for_album.md)
- [new_for_api](../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_api.md)
- [fetch_and_cache_content](../../../functions/src/wikipedia/database/fetch_and_cache_content.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [effective_default_catalog_code](../../../functions/src/config/Config/effective_default_catalog_code.md)
- [ingest_colnect_data_with_default_catalog](../../../functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [with_album_config_override](../../../functions/src/config/with_album_config_override.md)
- [finish_with_message](../../../functions/src/progress/ProgressIndicator/finish_with_message.md)

# Called by

- [main](../../../functions/src/main.md)