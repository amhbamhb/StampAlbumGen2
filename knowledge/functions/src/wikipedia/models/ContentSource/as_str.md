---
type: Rust Method
title: as_str
resource: src/wikipedia/models.rs#L9-L14
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/extract_block_metadata_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_commemorative_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_special_stamps_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_dedupe_fingerprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/colnect_pdf_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/stampworld_debug/month_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/catalog_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/link_colnect_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/utils/month_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_default_sort_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/try_load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_stamps_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_from_obj
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/numeric_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/mime_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/same_origin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_stampworld_import_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/resolve_left_editor_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/save_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/extract_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_single_image_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_stamp_from_title_and_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_ordinary_stamp_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_commemorative_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/validation/validate_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/fetchers/fetch_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/data_collector/test_day_month_regex_unicode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn as_str(&self) -> &str`

# Called by

- [insert_stamp_from_struct](../../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)
- [extract_block_metadata_local](../../../../../functions/crates/stamp_album_db/src/extract_block_metadata_local.md)
- [insert_stamp](../../../../../functions/crates/stamp_album_db/src/insert_stamp.md)
- [emit_commemorative_pages](../../../../../functions/src/album_globals/commemorative_generator/emit_commemorative_pages.md)
- [emit_special_stamps_page](../../../../../functions/src/album_globals/commemorative_generator/emit_special_stamps_page.md)
- [stamp_dedupe_fingerprints](../../../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)
- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [emit_stamp_grid_row](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row.md)
- [resolve_stamp_heading_label](../../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [estimate_and_ensure_placeholders](../../../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [colnect_pdf_options](../../../../../functions/src/album_globals/selection/colnect_pdf_options.md)
- [write_globals](../../../../../functions/src/album_utils/globals/write_globals.md)
- [main](../../../../../functions/src/bin/backfill_image_dims/main.md)
- [month_number](../../../../../functions/src/bin/stampworld_debug/month_number.md)
- [normalize_catalog_type](../../../../../functions/src/catalog/normalize_catalog_type.md)
- [catalog_display_name](../../../../../functions/src/catalog/catalog_display_name.md)
- [link_colnect_data](../../../../../functions/src/colnect/ingest/link_colnect_data.md)
- [month_number](../../../../../functions/src/commands_file_generator/utils/month_number.md)
- [review_ui_default_sort_mode](../../../../../functions/src/config/Config/review_ui_default_sort_mode.md)
- [try_load_slug_overrides](../../../../../functions/src/config/try_load_slug_overrides.md)
- [collect_stampworld_data](../../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [migrate_stamps_table](../../../../../functions/src/db_utils/migrations/migrate_stamps_table.md)
- [migrate_fix_block_metadata](../../../../../functions/src/db_utils/migrations/migrate_fix_block_metadata.md)
- [validate](../../../../../functions/src/external_album/parser/validate.md)
- [validate_page](../../../../../functions/src/external_album/parser/validate_page.md)
- [extract_from_obj](../../../../../functions/src/flagdb/extract_from_obj.md)
- [ensure_coat_of_arms_with_fetcher](../../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [numeric_tokens](../../../../../functions/src/modes/numeric_tokens.md)
- [run_scraping_mode](../../../../../functions/src/modes/run_scraping_mode.md)
- [mime_from_path](../../../../../functions/src/review_ui/mime_from_path.md)
- [same_origin](../../../../../functions/src/review_ui/external_album_api/same_origin.md)
- [handle_stampworld_import_request](../../../../../functions/src/review_ui/http_handlers/handle_stampworld_import_request.md)
- [serve_review_requests](../../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [apply_matches](../../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [resolve_left_editor_image_path](../../../../../functions/src/review_ui/matcher_service/resolve_left_editor_image_path.md)
- [save_left_stamp](../../../../../functions/src/review_ui/matcher_service/save_left_stamp.md)
- [metadata](../../../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [parse_stampworld_text](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)
- [import_stampworld_file](../../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [ensure_linux_desktop_integration](../../../../../functions/src/setup/ensure_linux_desktop_integration.md)
- [collect_stampworld_data_with_fetcher](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [extract_block_metadata](../../../../../functions/src/stampworld/parser/extract_block_metadata.md)
- [process_1847us_page](../../../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_single_image_stamp](../../../../../functions/src/us1847/collector/parse_single_image_stamp.md)
- [parse_stamp_from_title_and_description](../../../../../functions/src/us1847/collector/parse_stamp_from_title_and_description.md)
- [parse_ordinary_stamp_line](../../../../../functions/src/us1847/collector/parse_ordinary_stamp_line.md)
- [parse_commemorative_line](../../../../../functions/src/us1847/collector/parse_commemorative_line.md)
- [validate_source](../../../../../functions/src/validation/validate_source.md)
- [fetch_wikipedia_content](../../../../../functions/src/wikipedia/fetchers/fetch_wikipedia_content.md)
- [fetch_and_save_summary_with_fetcher](../../../../../functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher.md)
- [test_day_month_regex_unicode](../../../../../functions/tests/data_collector/test_day_month_regex_unicode.md)