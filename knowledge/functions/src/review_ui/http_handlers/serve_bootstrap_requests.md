---
type: Rust Function
title: serve_bootstrap_requests
resource: src/review_ui/http_handlers.rs#L138-L1030
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/http_handlers/bundled_favicon_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/ping_response_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/current_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_bootstrap_albums_root_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/stampworld_job_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/save_runtime_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_albumeasy_cli_path_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/PdfGenerationGuard/try_acquire
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/normalize_album_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/with_album_config_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_local_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/run_review_ui_bootstrap
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn serve_bootstrap_requests(server: &Server, html: &str) -> BootstrapHandlersState`

# Calls

- [bundled_favicon_png](../../../../functions/src/review_ui/http_handlers/bundled_favicon_png.md)
- [ping_response_json](../../../../functions/src/review_ui/http_handlers/ping_response_json.md)
- [scan_bootstrap_countries](../../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [current_bootstrap_albums_root](../../../../functions/src/review_ui/current_bootstrap_albums_root.md)
- [set_bootstrap_albums_root_override](../../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [pick_bootstrap_albums_root_via_dialog](../../../../functions/src/review_ui/pick_bootstrap_albums_root_via_dialog.md)
- [stampworld_job_state](../../../../functions/src/review_ui/stampworld_job_state.md)
- [tail_current_log](../../../../functions/src/review_ui/tail_current_log.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)
- [start_bootstrap_stampworld_job](../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [load_config](../../../../functions/src/config/load_config.md)
- [effective_albumeasy_cli_path](../../../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [save_runtime_albumeasy_cli_path](../../../../functions/src/review_ui/save_runtime_albumeasy_cli_path.md)
- [pick_albumeasy_cli_path_via_dialog](../../../../functions/src/review_ui/pick_albumeasy_cli_path_via_dialog.md)
- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [save_full_config_values](../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [try_acquire](../../../../functions/src/review_ui/PdfGenerationGuard/try_acquire.md)
- [best_effort_prepare_images_for_pdf](../../../../functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf.md)
- [normalize_album_images_for_pdf](../../../../functions/src/image_downloader/normalize_album_images_for_pdf.md)
- [album_dir_for_ui](../../../../functions/src/review_ui/album_dir_for_ui.md)
- [with_album_config_override](../../../../functions/src/config/with_album_config_override.md)
- [generate_album_output_conn_with_pdf_override](../../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [open_local_path](../../../../functions/src/review_ui/open_local_path.md)
- [colnect_import_data](../../../../functions/src/review_ui/colnect_import_data.md)
- [pick_and_copy_colnect_file_via_dialog](../../../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)

# Called by

- [run_review_ui_bootstrap](../../../../functions/src/review_ui/run_review_ui_bootstrap.md)