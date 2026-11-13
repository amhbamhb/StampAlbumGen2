---
type: Rust Function
title: contains
resource: src/native_pdf/stamp_image.rs#L190-L195
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_stampworld_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_colnect_page_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_placeholder_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/normalize_currency_token_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/is_commemorative_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/is_effectively_duplicate_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/is_real_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/is_commemorative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/center_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/link_colnect_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/detect_duplicate_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/ensure_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_stamps_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/normalize_currency_token_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/score_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/date/DateFormat/format_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/date/DateSource/parse_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/position_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/ReviewGeneratePdfRequest/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/quote_for_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/probe_port
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/is_colnect_csv_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/create_bootstrap_country_or_theme
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/enrich_field_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/is_multiline_array_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_utils/is_real_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/confirmDestructiveActionCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/is_cargo_build_binary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/normalize_currency_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/normalize_stamps_issued
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/dedupe_details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/parser/extract_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_single_image_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_table_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_stamp_from_title_and_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/extract_design_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/fetchers/fetch_grokipedia_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/parsers/parse_wikipedia_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/utils/estimate_text_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_new_setting_usage/test_identify_missing_width_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_args/help_shows_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_args/no_args_prints_guidance_and_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_args/scrape_mode_missing_site_name_reports_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_param_matrix/cli_param_matrix_core_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_template_key_usage_audit/parse_template_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_template_key_usage_audit/config_template_key_usage_audit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/make_stamp_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn contains(outer: Bounds, inner: Bounds) -> bool`

# Called by

- [insert_stamp_from_struct](../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)
- [is_stampworld_image_url](../../../../functions/crates/stamp_album_db/src/is_stampworld_image_url.md)
- [is_colnect_page_url](../../../../functions/crates/stamp_album_db/src/is_colnect_page_url.md)
- [is_colnect_image_url](../../../../functions/crates/stamp_album_db/src/is_colnect_image_url.md)
- [is_placeholder_image_path](../../../../functions/crates/stamp_album_db/src/is_placeholder_image_path.md)
- [normalize_currency_token_inner](../../../../functions/crates/stamp_album_db/src/normalize_currency_token_inner.md)
- [migrate_fix_block_metadata](../../../../functions/crates/stamp_album_db/src/migrate_fix_block_metadata.md)
- [insert_stamp](../../../../functions/crates/stamp_album_db/src/insert_stamp.md)
- [is_commemorative_series](../../../../functions/src/album_globals/commemorative_generator/is_commemorative_series.md)
- [is_effectively_duplicate_text](../../../../functions/src/album_globals/generator/is_effectively_duplicate_text.md)
- [is_real_image_path](../../../../functions/src/album_globals/generator/is_real_image_path.md)
- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [is_commemorative](../../../../functions/src/album_globals/generator_new/commemorative_layout/is_commemorative.md)
- [center_page](../../../../functions/src/album_globals/generator_new/postprocess/center_page.md)
- [calculate_stamp_dimensions](../../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [write_globals](../../../../functions/src/album_utils/globals/write_globals.md)
- [main](../../../../functions/src/bin/inspect_image_paths/main.md)
- [normalize_catalog_number](../../../../functions/src/catalog/normalize_catalog_number.md)
- [link_colnect_data](../../../../functions/src/colnect/ingest/link_colnect_data.md)
- [get_touchstamps_stamps](../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)
- [detect_duplicate_keys](../../../../functions/src/config/detect_duplicate_keys.md)
- [collect_stampworld_data](../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)
- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [migrate_stamps_table](../../../../functions/src/db_utils/migrations/migrate_stamps_table.md)
- [normalize_currency_token_inner](../../../../functions/src/db_utils/migrations/normalize_currency_token_inner.md)
- [migrate_fix_block_metadata](../../../../functions/src/db_utils/migrations/migrate_fix_block_metadata.md)
- [relative](../../../../functions/src/external_album/relative.md)
- [scan_inventory](../../../../functions/src/external_album/scan_inventory.md)
- [inspect](../../../../functions/src/external_album/inspect.md)
- [persist_pages](../../../../functions/src/external_album/persist_pages.md)
- [validate](../../../../functions/src/external_album/parser/validate.md)
- [ensure_coat_of_arms_with_fetcher](../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)
- [score_candidate](../../../../functions/src/modes/score_candidate.md)
- [load](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/load.md)
- [format_for](../../../../functions/src/native_pdf/date/DateFormat/format_for.md)
- [parse_for](../../../../functions/src/native_pdf/date/DateSource/parse_for.md)
- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [position_text](../../../../functions/src/native_pdf/stamp_image/position_text.md)
- [tail_current_log](../../../../functions/src/review_ui/tail_current_log.md)
- [validate](../../../../functions/src/review_ui/ReviewGeneratePdfRequest/validate.md)
- [quote_for_log](../../../../functions/src/review_ui/quote_for_log.md)
- [probe_port](../../../../functions/src/review_ui/probe_port.md)
- [is_colnect_csv_file](../../../../functions/src/review_ui/is_colnect_csv_file.md)
- [create_bootstrap_country_or_theme](../../../../functions/src/review_ui/create_bootstrap_country_or_theme.md)
- [store_uploaded_image_from_data_url](../../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)
- [enrich_field_help](../../../../functions/src/review_ui/config_editor/enrich_field_help.md)
- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [is_multiline_array_start](../../../../functions/src/review_ui/config_editor/is_multiline_array_start.md)
- [render_config_from_template_values](../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)
- [is_real_image_path](../../../../functions/src/review_ui/http_utils/is_real_image_path.md)
- [parse_stampworld_text](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)
- [import_stampworld_file](../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [confirmDestructiveActionCompare](../../../../functions/src/review_ui/views/compare/confirmDestructiveActionCompare.md)
- [is_cargo_build_binary](../../../../functions/src/setup/is_cargo_build_binary.md)
- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [normalize_currency_token](../../../../functions/src/stampworld/data_collector/normalize_currency_token.md)
- [normalize_stamps_issued](../../../../functions/src/stampworld/parser/normalize_stamps_issued.md)
- [dedupe_details](../../../../functions/src/stampworld/parser/dedupe_details.md)
- [extract_block_metadata](../../../../functions/src/stampworld/parser/extract_block_metadata.md)
- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)
- [parse_single_image_stamp](../../../../functions/src/us1847/collector/parse_single_image_stamp.md)
- [parse_table_stamps](../../../../functions/src/us1847/collector/parse_table_stamps.md)
- [parse_stamp_from_title_and_description](../../../../functions/src/us1847/collector/parse_stamp_from_title_and_description.md)
- [extract_design_size](../../../../functions/src/us1847/collector/extract_design_size.md)
- [parse_liberty_issue_stamp](../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)
- [fetch_grokipedia_content](../../../../functions/src/wikipedia/fetchers/fetch_grokipedia_content.md)
- [parse_wikipedia_html](../../../../functions/src/wikipedia/parsers/parse_wikipedia_html.md)
- [estimate_text_height](../../../../functions/src/wikipedia/utils/estimate_text_height.md)
- [test_identify_missing_width_usage](../../../../functions/tests/album_stamp_img_new_setting_usage/test_identify_missing_width_usage.md)
- [check_text](../../../../functions/tests/albumeasy_baseline/check_text.md)
- [help_shows_options](../../../../functions/tests/cli_args/help_shows_options.md)
- [no_args_prints_guidance_and_help](../../../../functions/tests/cli_args/no_args_prints_guidance_and_help.md)
- [scrape_mode_missing_site_name_reports_error](../../../../functions/tests/cli_args/scrape_mode_missing_site_name_reports_error.md)
- [cli_param_matrix_core_cases](../../../../functions/tests/cli_param_matrix/cli_param_matrix_core_cases.md)
- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)
- [test_create_config_from_template_force_backups_and_overwrites](../../../../functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites.md)
- [test_first_run_without_terminal_creates_defaults](../../../../functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults.md)
- [parse_template_keys](../../../../functions/tests/config_template_key_usage_audit/parse_template_keys.md)
- [config_template_key_usage_audit](../../../../functions/tests/config_template_key_usage_audit/config_template_key_usage_audit.md)
- [e2e_collect_stampworld_inserts_placeholders_to_db](../../../../functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [explicit_config_disabled_compatibility_and_templates](../../../../functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates.md)
- [make_stamp_image](../../../../functions/tests/native_pdf_stamp_image/make_stamp_image.md)