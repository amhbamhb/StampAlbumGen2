---
type: Rust Function
title: load_config
resource: src/config.rs#L1678-L2476
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/detect_duplicate_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/commands_file_generator/album_creation/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/sync_missing_stamps_from_colnect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/compute_placeholder_size_for_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/load_config_from_tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/load_example_config/test_load_example_config_roundtrip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/wikipedia_and_config_tests/config_template_is_copied_to_config_toml_when_missing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_config(path: &str) -> Config`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [detect_duplicate_keys](../../../functions/src/config/detect_duplicate_keys.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [with_defaults](../../../functions/src/config/Config/with_defaults.md)
- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)

# Called by

- [write_globals](../../../functions/src/commands_file_generator/album_creation/write_globals.md)
- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [load_config_with_override](../../../functions/src/config/load_config_with_override.md)
- [resolve_runtime_albums_root](../../../functions/src/file_utils/resolve_runtime_albums_root.md)
- [main](../../../functions/src/main.md)
- [resolve_runtime_albums_root](../../../functions/src/modes/resolve_runtime_albums_root.md)
- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [sync_missing_stamps_from_colnect](../../../functions/src/modes/sync_missing_stamps_from_colnect.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [compute_placeholder_size_for_series](../../../functions/src/placeholder_image/compute_placeholder_size_for_series.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [collect_stampworld_data_with_fetcher](../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [load_config_from_tempdir](../../../functions/tests/config_parameter_validation/load_config_from_tempdir.md)
- [e2e_collect_stampworld_inserts_placeholders_to_db](../../../functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db.md)
- [test_load_example_config_roundtrip](../../../functions/tests/load_example_config/test_load_example_config_roundtrip.md)
- [config_template_is_copied_to_config_toml_when_missing](../../../functions/tests/wikipedia_and_config_tests/config_template_is_copied_to_config_toml_when_missing.md)