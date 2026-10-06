---
type: Rust Function
title: serve_review_requests
resource: src/review_ui/http_handlers.rs#L1486-L2890
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/http_handlers/handle_site_account_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/bundled_favicon_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/respond
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/ping_response_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/stampworld_job_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/tail_current_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/build_config_editor_data_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/parse_query_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/build_matcher_payload
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/parse_query_param
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/mime_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/resolve_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/save_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/delete_left_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/clear_left_colnect_link
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/delete_colnect_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/undo_colnect_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/colnect_pdf_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/review_pdf_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_for_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/update_default_catalog_code_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_full_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/PdfGenerationGuard/try_acquire
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/ColnectPdfFilters/is_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/prepare_pdf_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/with_album_config_override_async
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_prepared_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_local_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_folder_in_file_manager
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn serve_review_requests( server: &Server, input: ReviewHandlersStateRequests, ) -> ReviewHandlersState`

# Calls

- [handle_site_account_request](../../../../functions/src/review_ui/http_handlers/handle_site_account_request.md)
- [handle_country_url_request](../../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)
- [bundled_favicon_png](../../../../functions/src/review_ui/http_handlers/bundled_favicon_png.md)
- [respond](../../../../functions/src/review_ui/external_album_api/respond.md)
- [ping_response_json](../../../../functions/src/review_ui/http_handlers/ping_response_json.md)
- [stampworld_job_state](../../../../functions/src/review_ui/stampworld_job_state.md)
- [tail_current_log](../../../../functions/src/review_ui/tail_current_log.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)
- [start_bootstrap_stampworld_job](../../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [album_dir_for_ui](../../../../functions/src/review_ui/album_dir_for_ui.md)
- [build_config_editor_data_for_album](../../../../functions/src/review_ui/config_editor/build_config_editor_data_for_album.md)
- [parse_query_flag](../../../../functions/src/review_ui/parse_query_flag.md)
- [build_matcher_payload](../../../../functions/src/review_ui/build_matcher_payload.md)
- [parse_query_param](../../../../functions/src/review_ui/parse_query_param.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [open](../../../../functions/src/external_album/open.md)
- [mime_from_path](../../../../functions/src/review_ui/mime_from_path.md)
- [resolve_colnect_image_url](../../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [apply_matches](../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [save_left_stamp](../../../../functions/src/review_ui/matcher_service/save_left_stamp.md)
- [add_left_stamp](../../../../functions/src/review_ui/matcher_service/add_left_stamp.md)
- [delete_left_stamp](../../../../functions/src/review_ui/matcher_service/delete_left_stamp.md)
- [clear_left_colnect_link](../../../../functions/src/review_ui/matcher_service/clear_left_colnect_link.md)
- [add_colnect_stamp_to_stamps](../../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [delete_colnect_candidate](../../../../functions/src/review_ui/matcher_service/delete_colnect_candidate.md)
- [undo_colnect_delete](../../../../functions/src/review_ui/matcher_service/undo_colnect_delete.md)
- [colnect_import_data](../../../../functions/src/review_ui/colnect_import_data.md)
- [colnect_pdf_options](../../../../functions/src/album_globals/selection/colnect_pdf_options.md)
- [review_pdf_error](../../../../functions/src/review_ui/http_handlers/review_pdf_error.md)
- [load_right_candidates](../../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [load_left_stamps](../../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [pick_and_copy_colnect_file_for_db](../../../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)
- [update_default_catalog_code_for_album](../../../../functions/src/review_ui/config_editor/update_default_catalog_code_for_album.md)
- [save_album_config_diff](../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)
- [save_full_config_values](../../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [try_acquire](../../../../functions/src/review_ui/PdfGenerationGuard/try_acquire.md)
- [is_all](../../../../functions/src/album_globals/selection/ColnectPdfFilters/is_all.md)
- [prepare_pdf_stamps](../../../../functions/src/album_globals/selection/prepare_pdf_stamps.md)
- [best_effort_prepare_images_for_pdf](../../../../functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf.md)
- [with_album_config_override_async](../../../../functions/src/config/with_album_config_override_async.md)
- [generate_album_output_conn_with_prepared_pdf](../../../../functions/src/album_globals/generate_album_output_conn_with_prepared_pdf.md)
- [generate_album_output_conn_with_pdf_override](../../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)
- [open_local_path](../../../../functions/src/review_ui/open_local_path.md)
- [open_folder_in_file_manager](../../../../functions/src/review_ui/open_folder_in_file_manager.md)

# Called by

- [run_review_ui](../../../../functions/src/review_ui/run_review_ui.md)
- [review_pdf_filters_routes_validate_requests_and_report_empty_selection](../../../../functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection.md)
- [country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings](../../../../functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings.md)
- [site_accounts_routes_return_json_in_bootstrap_and_review_modes](../../../../functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes.md)