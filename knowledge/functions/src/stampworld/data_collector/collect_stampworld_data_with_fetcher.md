---
type: Rust Function
title: collect_stampworld_data_with_fetcher
resource: src/stampworld/data_collector.rs#L30-L1531
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/dedupe_details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/generator/CommandsFileGenerator/get_stampworld_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/extract_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/compress_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/normalize_currency_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/normalize_stamps_issued
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/persist_image_dimensions_from_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/bin/stampworld_debug/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/db_path_unit/collector_creates_db_at_expected_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/fetcher_injection/test_collect_with_fake_fetcher_returns_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_stampworld_data_with_fetcher( fetcher: &dyn crate::stampworld::fetch::Fetcher, sequencer: &mut i32, _separator: &str, site_name: &str, images_directory: &str, month_number: &dyn Fn(&str) -> String, get_stampworld_image: &dyn Fn(&str, &str, bool) -> ImageStatus, base_url: &str, start_year: i32, end_year: i32, overwrite_images: bool, page_callback: Option<&dyn Fn(usize)>, max_pages: i32, stop_after_no_hits: i32, ) -> Vec<String>`

# Calls

- [load_config](../../../../functions/src/config/load_config.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [dedupe_details](../../../../functions/src/stampworld/parser/dedupe_details.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [sanitize_filename](../../../../functions/src/album_utils/text_utils/sanitize_filename.md)
- [get_stampworld_image](../../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/get_stampworld_image.md)
- [extract_block_metadata](../../../../functions/src/stampworld/parser/extract_block_metadata.md)
- [compress_context](../../../../functions/src/stampworld/parser/compress_context.md)
- [normalize_currency_token](../../../../functions/src/stampworld/data_collector/normalize_currency_token.md)
- [normalize_stamps_issued](../../../../functions/src/stampworld/parser/normalize_stamps_issued.md)
- [persist_image_dimensions_from_file](../../../../functions/src/db_utils/updates/persist_image_dimensions_from_file.md)
- [get_image_dimensions](../../../../functions/src/image_utils/get_image_dimensions.md)

# Called by

- [main](../../../../functions/src/bin/stampworld_debug/main.md)
- [collect_stampworld_data](../../../../functions/src/stampworld/data_collector/collect_stampworld_data.md)
- [collector_creates_db_at_expected_path](../../../../functions/tests/db_path_unit/collector_creates_db_at_expected_path.md)
- [e2e_collect_stampworld_inserts_placeholders_to_db](../../../../functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db.md)
- [test_collect_with_fake_fetcher_returns_empty](../../../../functions/tests/fetcher_injection/test_collect_with_fake_fetcher_returns_empty.md)