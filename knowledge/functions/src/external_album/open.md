---
type: Rust Function
title: open
resource: src/external_album.rs#L558-L567
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/schema
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/crates/stamp_album_db/src/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_fix_block_metadata/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/utils/is_valid_jpeg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/cache_country_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/normalize_album_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/file_sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/load_fallback_font
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/stampworld_db_snapshot
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_local_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/try_open_url_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_colnect_candidates_in_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/store_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/write_startup_probe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/fetch_and_cache_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/database/get_cached_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/inspect_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/ensure_coa_integration/ensure_coa_from_local_svg_creates_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/Fixture/db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/source_checksums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/frozen_reference_hashes_and_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/all_proof_policy_controls_actual_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/serialized_numbers_tokens_and_footer_coexistence_match_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/frozen_reference_identities_and_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/serialized_headers_and_title_interactions_match_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/four_page_sizes_render_and_read_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/page_break_overflow/overflowing_row_moves_to_next_page_and_emits_row_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/page_fit_regression/page_fit_regression_for_conservative_buffer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/imports_real_pdfs_into_disk_database_and_skips_reimports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_image_size/placeholder_per_stamp_has_expected_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_regeneration/placeholder_regeneration_forced
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stampworld_pdf_cli/cli_import_targets_explicit_db_instead_of_output_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stampworld_pdf_cli/executable_routes_pdf_with_db_path_to_import_not_album_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn open(root: &Path, name: &str) -> Result<(PathBuf, Connection)>`

# Calls

- [paths](../../../functions/src/external_album/paths.md)
- [err](../../../functions/src/external_album/err.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [schema](../../../functions/src/external_album/schema.md)

# Called by

- [init_db](../../../functions/crates/stamp_album_db/src/init_db.md)
- [add_wikipedia_section](../../../functions/src/album_utils/globals/add_wikipedia_section.md)
- [main](../../../functions/src/bin/inspect_image_paths/main.md)
- [main](../../../functions/src/bin/migrate_fix_block_metadata/main.md)
- [parse_csv](../../../functions/src/colnect/parser/parse_csv.md)
- [is_valid_jpeg](../../../functions/src/commands_file_generator/utils/is_valid_jpeg.md)
- [init_db](../../../functions/src/db_utils/init/init_db.md)
- [digest](../../../functions/src/external_album/digest.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [load_album](../../../functions/src/external_album/load_album.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)
- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [cache_country_info](../../../functions/src/flagdb/cache_country_info.md)
- [test_try_rasterize_svg_to_png_roundtrip](../../../functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip.md)
- [valid_svg_is_rendered_to_a_real_png_at_the_requested_size](../../../functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size.md)
- [normalize_album_images_for_pdf](../../../functions/src/image_downloader/normalize_album_images_for_pdf.md)
- [get_image_dimensions](../../../functions/src/image_utils/get_image_dimensions.md)
- [file_sha256](../../../functions/src/modes/file_sha256.md)
- [load_fallback_font](../../../functions/src/placeholder_image/load_fallback_font.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [tail_current_log](../../../functions/src/review_ui/tail_current_log.md)
- [stampworld_db_snapshot](../../../functions/src/review_ui/stampworld_db_snapshot.md)
- [open_local_path](../../../functions/src/review_ui/open_local_path.md)
- [try_open_url_once](../../../functions/src/review_ui/try_open_url_once.md)
- [log_startup](../../../functions/src/review_ui/log_startup.md)
- [count_colnect_candidates_in_db](../../../functions/src/review_ui/count_colnect_candidates_in_db.md)
- [store_image](../../../functions/src/review_ui/external_album_api/store_image.md)
- [handle](../../../functions/src/review_ui/external_album_api/handle.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [write_startup_probe](../../../functions/src/setup/write_startup_probe.md)
- [fetch_and_cache_content](../../../functions/src/wikipedia/database/fetch_and_cache_content.md)
- [get_cached_content](../../../functions/src/wikipedia/database/get_cached_content.md)
- [inspect_text](../../../functions/tests/albumeasy_baseline/inspect_text.md)
- [ensure_coa_from_local_svg_creates_png](../../../functions/tests/ensure_coa_integration/ensure_coa_from_local_svg_creates_png.md)
- [db](../../../functions/tests/external_album_backend/Fixture/db.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [source_checksums](../../../functions/tests/external_album_backend/source_checksums.md)
- [selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected](../../../functions/tests/external_album_backend/selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected.md)
- [named_colours_defaults_aliases_and_frozen_identities_match](../../../functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match.md)
- [explicit_file_changes_preserve_precision_even_margins_and_native_output](../../../functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output.md)
- [frozen_reference_hashes_and_media_sizes](../../../functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [frozen_reference_hashes_and_text](../../../functions/tests/native_pdf_decorative_border/frozen_reference_hashes_and_text.md)
- [actual_decorative_pixels_and_transparency_match](../../../functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match.md)
- [all_proof_policy_controls_actual_output](../../../functions/tests/native_pdf_explicit_settings/all_proof_policy_controls_actual_output.md)
- [serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry](../../../functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [serialized_numbers_tokens_and_footer_coexistence_match_references](../../../functions/tests/native_pdf_footer_number/serialized_numbers_tokens_and_footer_coexistence_match_references.md)
- [serialized_dates_and_coexistence_match_frozen_reference_pdfs](../../../functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs.md)
- [generation_date_requires_one_explicit_document_context](../../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)
- [serialized_header_numbers_match_reference_fonts_glyphs_and_geometry](../../../functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry.md)
- [frozen_reference_identities_and_text](../../../functions/tests/native_pdf_page_border/frozen_reference_identities_and_text.md)
- [serialized_footer_positions_colours_and_content_match_references](../../../functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references.md)
- [serialized_headers_and_title_interactions_match_references](../../../functions/tests/native_pdf_page_header/serialized_headers_and_title_interactions_match_references.md)
- [four_page_sizes_render_and_read_back](../../../functions/tests/native_pdf_page_settings/four_page_sizes_render_and_read_back.md)
- [title_pdf_positions_match_frozen_references_and_hint_controls_guide](../../../functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide.md)
- [stamp_image_heading_caption_and_page_border_render_with_physical_geometry](../../../functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)
- [overflowing_row_moves_to_next_page_and_emits_row_start](../../../functions/tests/page_break_overflow/overflowing_row_moves_to_next_page_and_emits_row_start.md)
- [page_fit_regression_for_conservative_buffer](../../../functions/tests/page_fit_regression/page_fit_regression_for_conservative_buffer.md)
- [imports_real_pdfs_into_disk_database_and_skips_reimports](../../../functions/tests/pdf_import_tool/imports_real_pdfs_into_disk_database_and_skips_reimports.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)
- [spec_placeholder_inverts_for_dark_stamp_background](../../../functions/tests/placeholder_image_inversion_spec/spec_placeholder_inverts_for_dark_stamp_background.md)
- [placeholder_series_size_matches_hint](../../../functions/tests/placeholder_image_series_size/placeholder_series_size_matches_hint.md)
- [placeholder_per_stamp_has_expected_dimensions](../../../functions/tests/placeholder_image_size/placeholder_per_stamp_has_expected_dimensions.md)
- [placeholder_regeneration_forced](../../../functions/tests/placeholder_regeneration/placeholder_regeneration_forced.md)
- [cli_import_targets_explicit_db_instead_of_output_only_mode](../../../functions/tests/stampworld_pdf_cli/cli_import_targets_explicit_db_instead_of_output_only_mode.md)
- [executable_routes_pdf_with_db_path_to_import_not_album_generation](../../../functions/tests/stampworld_pdf_cli/executable_routes_pdf_with_db_path_to_import_not_album_generation.md)
- [wikipedia_ingest_inserts_up_to_max_entries_from_cache](../../../functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache.md)