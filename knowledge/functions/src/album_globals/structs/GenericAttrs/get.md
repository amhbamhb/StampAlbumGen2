---
type: Rust Method
title: get
resource: src/album_globals/structs.rs#L24-L26
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/extract_block_metadata_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/get_stamps_by_country_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/tests/unit_tests/test_insert_stamp_and_backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/tests/unit_tests/test_migrate_fix_block_metadata_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/is_commemorative_for_year
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/is_special_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_commemorative_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_special_stamps_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_summary_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_date_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_dedupe_fingerprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/is_commemorative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/load_stored_colnect_meta
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/db_loader/load_stamps_from_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/process_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_stamps/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/load_colnect_info_with_query
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/hydrate_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/get_colnect_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/download_to_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/image_utils/get_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/deep_merge_toml_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migration_002_stamps_uniq_key_and_indexes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/enforce_runtime_schema_compatibility
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_stamps_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/insert_stamp_placeholder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/get_stamps_by_country_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/country_to_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/extract_year
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_colnect_candidates_in_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/sync_processed_colnect_candidates_marks_existing_stamp_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/mark_colnect_candidate_processed_sets_flag_for_matching_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/enrich_field_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/fetch_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/fetch_bytes
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
  - target: functions/src/us1847/collector/parse_table_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_stamp_from_title_and_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_ordinary_stamp_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_commemorative_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/download_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/fetchers/fetch_wikipedia_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/ingest/ingest_wikipedia_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/api_reexports/insert_stamp_placeholder_via_crate_reexport
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/data_collector/test_day_month_regex_unicode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/db_utils/test_insert_stamp_placeholder_allows_distinct_sg_numbers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/db_utils/test_migrate_fix_block_metadata_basic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get(&self, key: &str) -> Option<&String>`

# Called by

- [insert_stamp_placeholder](../../../../../functions/crates/stamp_album_db/src/insert_stamp_placeholder.md)
- [extract_block_metadata_local](../../../../../functions/crates/stamp_album_db/src/extract_block_metadata_local.md)
- [migrate_fix_block_metadata](../../../../../functions/crates/stamp_album_db/src/migrate_fix_block_metadata.md)
- [insert_stamp](../../../../../functions/crates/stamp_album_db/src/insert_stamp.md)
- [get_stamps_by_country_series](../../../../../functions/crates/stamp_album_db/src/get_stamps_by_country_series.md)
- [test_insert_stamp_and_backfill](../../../../../functions/crates/stamp_album_db/tests/unit_tests/test_insert_stamp_and_backfill.md)
- [test_migrate_fix_block_metadata_local](../../../../../functions/crates/stamp_album_db/tests/unit_tests/test_migrate_fix_block_metadata_local.md)
- [is_commemorative_for_year](../../../../../functions/src/album_globals/commemorative_generator/is_commemorative_for_year.md)
- [is_special_stamp](../../../../../functions/src/album_globals/commemorative_generator/is_special_stamp.md)
- [emit_commemorative_pages](../../../../../functions/src/album_globals/commemorative_generator/emit_commemorative_pages.md)
- [emit_special_stamps_page](../../../../../functions/src/album_globals/commemorative_generator/emit_special_stamps_page.md)
- [emit_summary_page](../../../../../functions/src/album_globals/commemorative_generator/emit_summary_page.md)
- [stamp_date_key](../../../../../functions/src/album_globals/generator/stamp_date_key.md)
- [stamp_dedupe_fingerprints](../../../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)
- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [is_commemorative](../../../../../functions/src/album_globals/generator_new/commemorative_layout/is_commemorative.md)
- [emit_stamp_grid_row](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row.md)
- [build_stamp_description](../../../../../functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description.md)
- [load_stored_colnect_meta](../../../../../functions/src/album_globals/generator_new/helpers/load_stored_colnect_meta.md)
- [set_attr_if_missing](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_missing.md)
- [fetch_and_filter_stamps](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [estimate_and_ensure_placeholders](../../../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [load_stamps_from_db](../../../../../functions/src/album_utils/db_loader/load_stamps_from_db.md)
- [calculate_stamp_dimensions](../../../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [process_db](../../../../../functions/src/bin/backfill_image_dims/process_db.md)
- [main](../../../../../functions/src/bin/backfill_image_dims/main.md)
- [main](../../../../../functions/src/bin/migrate_stamps/main.md)
- [load_colnect_info_with_query](../../../../../functions/src/colnect/db/load_colnect_info_with_query.md)
- [hydrate_catalog_codes](../../../../../functions/src/colnect/db/hydrate_catalog_codes.md)
- [get_colnect_info](../../../../../functions/src/colnect/db/get_colnect_info.md)
- [download_to_path](../../../../../functions/src/commands_file_generator/image_utils/download_to_path.md)
- [get_image](../../../../../functions/src/commands_file_generator/image_utils/get_image.md)
- [get_touchstamps_stamps](../../../../../functions/src/commands_file_generator/touchstamps/get_touchstamps_stamps.md)
- [load_config](../../../../../functions/src/config/load_config.md)
- [deep_merge_toml_values](../../../../../functions/src/config/deep_merge_toml_values.md)
- [load_config_with_override](../../../../../functions/src/config/load_config_with_override.md)
- [load_slug_overrides](../../../../../functions/src/config/load_slug_overrides.md)
- [collect_stampworld_data](../../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [migration_002_stamps_uniq_key_and_indexes](../../../../../functions/src/db_utils/migrations/migration_002_stamps_uniq_key_and_indexes.md)
- [enforce_runtime_schema_compatibility](../../../../../functions/src/db_utils/migrations/enforce_runtime_schema_compatibility.md)
- [migrate_stamps_table](../../../../../functions/src/db_utils/migrations/migrate_stamps_table.md)
- [run_pending_schema_migrations_adds_missing_columns_and_records_versions](../../../../../functions/src/db_utils/migrations/run_pending_schema_migrations_adds_missing_columns_and_records_versions.md)
- [migrate_fix_block_metadata](../../../../../functions/src/db_utils/migrations/migrate_fix_block_metadata.md)
- [insert_stamp](../../../../../functions/src/db_utils/operations/insert_stamp.md)
- [insert_stamp_placeholder](../../../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)
- [get_stamps_by_country_series](../../../../../functions/src/db_utils/operations/get_stamps_by_country_series.md)
- [country_to_code](../../../../../functions/src/flagdb/country_to_code.md)
- [ensure_coat_of_arms_with_fetcher](../../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [download_image](../../../../../functions/src/image_downloader/download_image.md)
- [extract_year](../../../../../functions/src/modes/extract_year.md)
- [run_scraping_mode](../../../../../functions/src/modes/run_scraping_mode.md)
- [count_colnect_candidates_in_db](../../../../../functions/src/review_ui/count_colnect_candidates_in_db.md)
- [sync_processed_colnect_candidates_marks_existing_stamp_links](../../../../../functions/src/review_ui/sync_processed_colnect_candidates_marks_existing_stamp_links.md)
- [mark_colnect_candidate_processed_sets_flag_for_matching_candidate](../../../../../functions/src/review_ui/mark_colnect_candidate_processed_sets_flag_for_matching_candidate.md)
- [enrich_field_help](../../../../../functions/src/review_ui/config_editor/enrich_field_help.md)
- [build_config_editor_data](../../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [render_config_from_template_values](../../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)
- [save_full_config_values](../../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [build_config_editor_data_for_album](../../../../../functions/src/review_ui/config_editor/build_config_editor_data_for_album.md)
- [save_album_config_diff](../../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)
- [serve_review_requests](../../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [load_left_stamps](../../../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [apply_matches](../../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [load_left_stamp_by_rowid](../../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)
- [load_right_candidate_by_colnect_id](../../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)
- [collect_stampworld_data_with_fetcher](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [fetch_text](../../../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_bytes.md)
- [fetch_text](../../../../../functions/src/stampworld/fetch/fetch_text.md)
- [fetch_bytes](../../../../../functions/src/stampworld/fetch/fetch_bytes.md)
- [extract_block_metadata](../../../../../functions/src/stampworld/parser/extract_block_metadata.md)
- [process_1847us_page](../../../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_single_image_stamp](../../../../../functions/src/us1847/collector/parse_single_image_stamp.md)
- [parse_table_stamps](../../../../../functions/src/us1847/collector/parse_table_stamps.md)
- [parse_stamp_from_title_and_description](../../../../../functions/src/us1847/collector/parse_stamp_from_title_and_description.md)
- [parse_ordinary_stamp_line](../../../../../functions/src/us1847/collector/parse_ordinary_stamp_line.md)
- [parse_liberty_issue_stamp](../../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)
- [parse_commemorative_line](../../../../../functions/src/us1847/collector/parse_commemorative_line.md)
- [download_stamp_images](../../../../../functions/src/us1847/collector/download_stamp_images.md)
- [fetch_and_cache_content](../../../../../functions/src/wikipedia/database/fetch_and_cache_content.md)
- [get_cached_content](../../../../../functions/src/wikipedia/database/get_cached_content.md)
- [fetch_wikipedia_content](../../../../../functions/src/wikipedia/fetchers/fetch_wikipedia_content.md)
- [ingest_wikipedia_to_db](../../../../../functions/src/wikipedia/ingest/ingest_wikipedia_to_db.md)
- [fetch_and_save_summary_with_fetcher](../../../../../functions/src/wikipedia/legacy/fetch_and_save_summary_with_fetcher.md)
- [insert_stamp_placeholder_via_crate_reexport](../../../../../functions/tests/api_reexports/insert_stamp_placeholder_via_crate_reexport.md)
- [backfill_image_dimensions_persists_to_db](../../../../../functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db.md)
- [test_day_month_regex_unicode](../../../../../functions/tests/data_collector/test_day_month_regex_unicode.md)
- [test_insert_stamp_placeholder_allows_distinct_sg_numbers](../../../../../functions/tests/db_utils/test_insert_stamp_placeholder_allows_distinct_sg_numbers.md)
- [test_migrate_fix_block_metadata_basic](../../../../../functions/tests/db_utils/test_migrate_fix_block_metadata_basic.md)
- [e2e_collect_stampworld_inserts_placeholders_to_db](../../../../../functions/tests/e2e_collect_stampworld_db/e2e_collect_stampworld_inserts_placeholders_to_db.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../../../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)
- [wikipedia_ingest_inserts_up_to_max_entries_from_cache](../../../../../functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache.md)