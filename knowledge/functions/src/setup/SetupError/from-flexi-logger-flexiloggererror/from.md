---
type: Rust Method
title: from
resource: src/setup.rs#L52-L54
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/maybe_generate_album_pdf_with_settings
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
  - target: functions/src/config/load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/init/init_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/find_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/load_flag_map_from_fs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/load_embedded_flag_map_if_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
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
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
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
  - target: functions/src/review_ui/http_handlers/serve_review_requests
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
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from(err: flexi_logger::FlexiLoggerError) -> Self`

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [generate_album_output_conn](../../../../../functions/src/album_globals/generator_new/generate_album_output_conn.md)
- [maybe_generate_album_pdf_with_settings](../../../../../functions/src/album_globals/maybe_generate_album_pdf_with_settings.md)
- [main](../../../../../functions/src/bin/backfill_image_dims/main.md)
- [main](../../../../../functions/src/bin/inspect_image_paths/main.md)
- [ensure_config_and_template_files](../../../../../functions/src/config/ensure_config_and_template_files.md)
- [load_config](../../../../../functions/src/config/load_config.md)
- [load_slug_overrides](../../../../../functions/src/config/load_slug_overrides.md)
- [collect_stampworld_data](../../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [init_db](../../../../../functions/src/db_utils/init/init_db.md)
- [resolve_runtime_albums_root](../../../../../functions/src/file_utils/resolve_runtime_albums_root.md)
- [find_config_path](../../../../../functions/src/file_utils/find_config_path.md)
- [load_flag_map_from_fs](../../../../../functions/src/flagdb/load_flag_map_from_fs.md)
- [load_embedded_flag_map_if_enabled](../../../../../functions/src/flagdb/load_embedded_flag_map_if_enabled.md)
- [ensure_flag_image_with_fetcher](../../../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [ensure_coat_of_arms_with_fetcher](../../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [resolve_runtime_albums_root](../../../../../functions/src/modes/resolve_runtime_albums_root.md)
- [run_colnect_inbox_mode](../../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../../functions/src/modes/run_scraping_mode.md)
- [draw_text](../../../../../functions/src/placeholder_image/draw_text.md)
- [start_bootstrap_stampworld_job](../../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [log_startup](../../../../../functions/src/review_ui/log_startup.md)
- [pick_and_copy_colnect_file_via_dialog](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [pick_and_copy_colnect_file_for_db](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)
- [pick_and_copy_colnect_file_to_dir](../../../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)
- [set_bootstrap_albums_root_override](../../../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [album_dir_for_ui](../../../../../functions/src/review_ui/album_dir_for_ui.md)
- [colnect_import_data](../../../../../functions/src/review_ui/colnect_import_data.md)
- [serve_review_requests](../../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [startup_probe_path](../../../../../functions/src/setup/startup_probe_path.md)
- [refresh_linux_desktop_caches](../../../../../functions/src/setup/refresh_linux_desktop_caches.md)
- [user_home_dir](../../../../../functions/src/setup/user_home_dir.md)
- [query_windows_desktop_dir](../../../../../functions/src/setup/query_windows_desktop_dir.md)
- [query_windows_programs_dir](../../../../../functions/src/setup/query_windows_programs_dir.md)
- [ensure_windows_desktop_integration](../../../../../functions/src/setup/ensure_windows_desktop_integration.md)
- [ensure_linux_desktop_integration](../../../../../functions/src/setup/ensure_linux_desktop_integration.md)
- [collect_stampworld_data_with_fetcher](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)