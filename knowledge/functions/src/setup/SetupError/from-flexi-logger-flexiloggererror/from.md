---
type: Rust Method
title: from
resource: src/setup.rs#L54-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/slug_override_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/find_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/flag_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/colour/NamedColour/rgb
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/numbering/PageNumbering/text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_for_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_to_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_utils/test_is_pdf_with_directory_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/openReviewAfterStampWorld
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/applyRightCatalogSearchChoice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/startup_probe_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/refresh_linux_desktop_caches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/user_home_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/query_windows_desktop_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/query_windows_programs_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_windows_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_first_run_installation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/test_decision_moves_if_first_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/multiword_names_and_explicit_mappings_build_the_expected_urls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_async
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_exact_acceptance_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from(err: flexi_logger::FlexiLoggerError) -> Self`

# Called by

- [build_stamp_description](../../../../../functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description.md)
- [maybe_generate_album_pdf_with_settings](../../../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [generated_artifacts_from_path](../../../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [main](../../../../../functions/src/bin/backfill_image_dims/main.md)
- [main](../../../../../functions/src/bin/inspect_image_paths/main.md)
- [ensure_config_and_template_files](../../../../../functions/src/config/ensure_config_and_template_files.md)
- [load_config](../../../../../functions/src/config/load_config.md)
- [slug_override_path](../../../../../functions/src/config/slug_override_path.md)
- [collect_stampworld_data](../../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [init_db](../../../../../functions/src/db_utils/init/init_db.md)
- [render](../../../../../functions/src/external_album/render.md)
- [generate_artifacts](../../../../../functions/src/external_album/generate_artifacts.md)
- [resolve_runtime_albums_root](../../../../../functions/src/file_utils/resolve_runtime_albums_root.md)
- [find_config_path](../../../../../functions/src/file_utils/find_config_path.md)
- [ensure_coat_of_arms_with_fetcher](../../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [flag_directories](../../../../../functions/src/flagdb/assets/flag_directories.md)
- [partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply](../../../../../functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply.md)
- [resolve_runtime_albums_root](../../../../../functions/src/modes/resolve_runtime_albums_root.md)
- [run_colnect_inbox_mode](../../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../../functions/src/modes/run_scraping_mode.md)
- [rgb](../../../../../functions/src/native_pdf/colour/NamedColour/rgb.md)
- [image_dimensions_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm.md)
- [text](../../../../../functions/src/native_pdf/numbering/PageNumbering/text.md)
- [prepare](../../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [embedded_reference_scale_matches_great_britain_albumeasy_footprints](../../../../../functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints.md)
- [draw_text](../../../../../functions/src/placeholder_image/draw_text.md)
- [start_bootstrap_stampworld_job](../../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [log_startup](../../../../../functions/src/review_ui/log_startup.md)
- [pick_and_copy_stampworld_file_via_dialog](../../../../../functions/src/review_ui/pick_and_copy_stampworld_file_via_dialog.md)
- [pick_and_copy_colnect_file_via_dialog](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [pick_and_copy_colnect_file_for_db](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)
- [pick_and_copy_colnect_file_to_dir](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)
- [set_bootstrap_albums_root_override](../../../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [album_dir_for_ui](../../../../../functions/src/review_ui/album_dir_for_ui.md)
- [colnect_import_data](../../../../../functions/src/review_ui/colnect_import_data.md)
- [renders_global_and_saves_album_config_with_quoted_windows_path](../../../../../functions/src/review_ui/config_editor/renders_global_and_saves_album_config_with_quoted_windows_path.md)
- [serve_review_requests](../../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [test_is_pdf_with_directory_path](../../../../../functions/src/review_ui/http_utils/test_is_pdf_with_directory_path.md)
- [pollStampWorldStatus](../../../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [openReviewAfterStampWorld](../../../../../functions/src/review_ui/views/bootstrap/openReviewAfterStampWorld.md)
- [applyRightCatalogSearchChoice](../../../../../functions/src/review_ui/views/compare/applyRightCatalogSearchChoice.md)
- [startup_probe_path](../../../../../functions/src/setup/startup_probe_path.md)
- [refresh_linux_desktop_caches](../../../../../functions/src/setup/refresh_linux_desktop_caches.md)
- [user_home_dir](../../../../../functions/src/setup/user_home_dir.md)
- [query_windows_desktop_dir](../../../../../functions/src/setup/query_windows_desktop_dir.md)
- [query_windows_programs_dir](../../../../../functions/src/setup/query_windows_programs_dir.md)
- [ensure_windows_desktop_integration](../../../../../functions/src/setup/ensure_windows_desktop_integration.md)
- [ensure_linux_desktop_integration](../../../../../functions/src/setup/ensure_linux_desktop_integration.md)
- [ensure_first_run_installation](../../../../../functions/src/setup/ensure_first_run_installation.md)
- [test_decision_moves_if_first_run](../../../../../functions/src/setup/test_decision_moves_if_first_run.md)
- [multiword_names_and_explicit_mappings_build_the_expected_urls](../../../../../functions/src/stampworld/country_url/multiword_names_and_explicit_mappings_build_the_expected_urls.md)
- [collect_stampworld_data_async](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_async.md)
- [collect_stampworld_data_with_fetcher](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)
- [brazil_exact_acceptance_counts](../../../../../functions/tests/external_album_backend/brazil_exact_acceptance_counts.md)
- [brazil_database_roundtrip_and_repeated_placements](../../../../../functions/tests/external_album_backend/brazil_database_roundtrip_and_repeated_placements.md)
- [brazil_real_albumeasy_import_approval_and_page_edit](../../../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)
- [serialized_footer_positions_colours_and_content_match_references](../../../../../functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references.md)