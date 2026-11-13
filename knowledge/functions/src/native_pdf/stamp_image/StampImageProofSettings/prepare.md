---
type: Rust Method
title: prepare
resource: src/native_pdf/stamp_image.rs#L96-L173
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/validate_numbers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/position_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/crates/stamp_album_db/src/insert_stamp_from_struct
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/load_existing_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/get_stamps_by_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/get_all_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/migrate_stamps_table
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
  - target: functions/src/album_utils/db_loader/load_stamps_from_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/process_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
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
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/link_colnect_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/table_column_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/cleanup_legacy_stampworld_identity_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_stamps_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/migrate_fix_block_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/migrations/column_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/get_stamps_by_country_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/get_stamps_by_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/get_all_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/has_import_checksum
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/remap_colnect_sg_to_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/natural_size_image_is_centered_in_padded_box_without_upscaling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_stamp_all_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/db_query/is_duplicate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/download_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/sync_us1847_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_colnect_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/db_utils/test_insert_stamp_backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/placement_identities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn prepare(&self, page: &PageLayout) -> Result<PreparedStampImage>`

# Calls

- [validate_numbers](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/validate_numbers.md)
- [contains](../../../../../functions/src/native_pdf/stamp_image/contains.md)
- [inset](../../../../../functions/src/native_pdf/document/Bounds/inset.md)
- [points_to_mm](../../../../../functions/src/native_pdf/units/points_to_mm.md)
- [from](../../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [mm_to_points](../../../../../functions/src/native_pdf/units/mm_to_points.md)
- [position_text](../../../../../functions/src/native_pdf/stamp_image/position_text.md)

# Called by

- [insert_stamp_from_struct](../../../../../functions/crates/stamp_album_db/src/insert_stamp_from_struct.md)
- [load_existing_stamp](../../../../../functions/crates/stamp_album_db/src/load_existing_stamp.md)
- [get_stamps_by_series](../../../../../functions/crates/stamp_album_db/src/get_stamps_by_series.md)
- [get_all_series](../../../../../functions/crates/stamp_album_db/src/get_all_series.md)
- [migrate_stamps_table](../../../../../functions/crates/stamp_album_db/src/migrate_stamps_table.md)
- [migrate_fix_block_metadata](../../../../../functions/crates/stamp_album_db/src/migrate_fix_block_metadata.md)
- [insert_stamp](../../../../../functions/crates/stamp_album_db/src/insert_stamp.md)
- [get_stamps_by_country_series](../../../../../functions/crates/stamp_album_db/src/get_stamps_by_country_series.md)
- [load_stamps_from_db](../../../../../functions/src/album_utils/db_loader/load_stamps_from_db.md)
- [add_wikipedia_section](../../../../../functions/src/album_utils/globals/add_wikipedia_section.md)
- [process_db](../../../../../functions/src/bin/backfill_image_dims/process_db.md)
- [main](../../../../../functions/src/bin/inspect_image_paths/main.md)
- [main](../../../../../functions/src/bin/migrate_stamps/main.md)
- [load_colnect_info_with_query](../../../../../functions/src/colnect/db/load_colnect_info_with_query.md)
- [hydrate_catalog_codes](../../../../../functions/src/colnect/db/hydrate_catalog_codes.md)
- [get_colnect_info](../../../../../functions/src/colnect/db/get_colnect_info.md)
- [ingest_colnect_data_with_mode](../../../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)
- [link_colnect_data](../../../../../functions/src/colnect/ingest/link_colnect_data.md)
- [table_column_names](../../../../../functions/src/db_utils/migrations/table_column_names.md)
- [cleanup_legacy_stampworld_identity_duplicates](../../../../../functions/src/db_utils/migrations/cleanup_legacy_stampworld_identity_duplicates.md)
- [run_pending_schema_migrations](../../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)
- [migrate_stamps_table](../../../../../functions/src/db_utils/migrations/migrate_stamps_table.md)
- [migrate_fix_block_metadata](../../../../../functions/src/db_utils/migrations/migrate_fix_block_metadata.md)
- [column_names](../../../../../functions/src/db_utils/migrations/column_names.md)
- [get_stamps_by_country_series](../../../../../functions/src/db_utils/operations/get_stamps_by_country_series.md)
- [get_stamps_by_series](../../../../../functions/src/db_utils/operations/get_stamps_by_series.md)
- [get_all_series](../../../../../functions/src/db_utils/operations/get_all_series.md)
- [details](../../../../../functions/src/external_album/details.md)
- [persist_pages](../../../../../functions/src/external_album/persist_pages.md)
- [verify_source](../../../../../functions/src/external_album/verify_source.md)
- [infer_and_fix_catalog_codes_for_colnect_rows](../../../../../functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows.md)
- [has_import_checksum](../../../../../functions/src/modes/has_import_checksum.md)
- [remap_colnect_sg_to_existing_stamps](../../../../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)
- [write_proof](../../../../../functions/src/native_pdf/renderer/write_proof.md)
- [natural_size_image_is_centered_in_padded_box_without_upscaling](../../../../../functions/src/native_pdf/stamp_image/natural_size_image_is_centered_in_padded_box_without_upscaling.md)
- [embedded_reference_scale_matches_great_britain_albumeasy_footprints](../../../../../functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints.md)
- [load_stamp_all_fields](../../../../../functions/src/review_ui/load_stamp_all_fields.md)
- [is_duplicate](../../../../../functions/src/review_ui/db_query/is_duplicate.md)
- [load_left_stamps](../../../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [load_left_stamp_by_rowid](../../../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)
- [load_right_candidate_by_colnect_id](../../../../../functions/src/review_ui/matcher_service/load_right_candidate_by_colnect_id.md)
- [download_stamp_images](../../../../../functions/src/us1847/collector/download_stamp_images.md)
- [sync_us1847_to_stamps](../../../../../functions/src/us1847/collector/sync_us1847_to_stamps.md)
- [fetch_and_cache_content](../../../../../functions/src/wikipedia/database/fetch_and_cache_content.md)
- [get_cached_content](../../../../../functions/src/wikipedia/database/get_cached_content.md)
- [test_colnect_integration](../../../../../functions/tests/colnect_integration/test_colnect_integration.md)
- [test_insert_stamp_backfill](../../../../../functions/tests/db_utils/test_insert_stamp_backfill.md)
- [placement_identities](../../../../../functions/tests/external_album_backend/placement_identities.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../../../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)