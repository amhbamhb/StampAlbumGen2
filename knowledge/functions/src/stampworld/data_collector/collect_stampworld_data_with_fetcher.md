---
type: Rust Function
title: collect_stampworld_data_with_fetcher
resource: src/stampworld/data_collector.rs#L125-L1622
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/html
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
  - target: functions/src/db_utils/operations/insert_stamp_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/text_utils/sanitize_filename
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

`pub async fn collect_stampworld_data_with_fetcher( fetcher: &dyn crate::stampworld::fetch::Fetcher, params: CollectStampworldParams<'_>, ) -> Vec<String>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [parse](../../../../functions/src/external_album/parser/parse.md)
- [html](../../../../functions/src/review_ui/views/compare/html.md)
- [dedupe_details](../../../../functions/src/stampworld/parser/dedupe_details.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [insert_stamp_placeholder](../../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)
- [sanitize_filename](../../../../functions/src/album_utils/text_utils/sanitize_filename.md)
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