---
type: Rust Module
title: review_ui
resource: src/review_ui.rs#L1-L3455
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/base64-engine
    resolved_by: tree-sitter
    confidence: exact
  - target: external/http-handlers-serve-bootstrap-requests-serve-review-requests-reviewhandlersstaterequests
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-vecdeque
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufread-bufreader
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command-stdio
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicbool-ordering
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex-oncelock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-thread
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tiny-http-header-method-response-server-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/views-render-bootstrap-html-render-html
    resolved_by: tree-sitter
    confidence: exact
  - target: external/config-editor-build-config-editor-data-build-config-editor-data-for-album-render-config-editor-assets-save-album-config-diff-save-full-config-values-update-default-catalog-code-for-album
    resolved_by: tree-sitter
    confidence: exact
  - target: external/matcher-service-add-colnect-stamp-to-stamps-add-left-stamp-apply-matches-clear-left-colnect-link-delete-colnect-candidate-delete-left-stamp-load-left-stamps-load-right-candidates-save-left-stamp-sync-processed-colnect-candidates-undo-colnect-delete
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-net-tcpstream
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-matcher-service-mark-colnect-candidate-processed-sync-processed-colnect-candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-parser
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [runtime_log_buffer](../../functions/src/review_ui/runtime_log_buffer.md)
- [stampworld_job_state](../../functions/src/review_ui/stampworld_job_state.md)
- [bootstrap_albums_root_override](../../functions/src/review_ui/bootstrap_albums_root_override.md)
- [sanitize_credentials_in_text](../../functions/src/review_ui/sanitize_credentials_in_text.md)
- [log_runtime_event](../../functions/src/review_ui/log_runtime_event.md)
- [get_runtime_log_lines](../../functions/src/review_ui/get_runtime_log_lines.md)
- [tail_current_log](../../functions/src/review_ui/tail_current_log.md)
- [PdfGenerationGuard](../../classes/src/review_ui/PdfGenerationGuard.md)
- [try_acquire](../../functions/src/review_ui/PdfGenerationGuard/try_acquire.md)
- [drop](../../functions/src/review_ui/PdfGenerationGuard/drop/drop.md)
- [DbField](../../classes/src/review_ui/DbField.md)
- [LeftStamp](../../classes/src/review_ui/LeftStamp.md)
- [RightCandidate](../../classes/src/review_ui/RightCandidate.md)
- [CatalogCodePair](../../classes/src/review_ui/CatalogCodePair.md)
- [MatcherPayload](../../classes/src/review_ui/MatcherPayload.md)
- [BootstrapCountryInfo](../../classes/src/review_ui/BootstrapCountryInfo.md)
- [BootstrapStateResponse](../../classes/src/review_ui/BootstrapStateResponse.md)
- [BootstrapStartRequest](../../classes/src/review_ui/BootstrapStartRequest.md)
- [BootstrapStartResponse](../../classes/src/review_ui/BootstrapStartResponse.md)
- [BootstrapCreateCountryRequest](../../classes/src/review_ui/BootstrapCreateCountryRequest.md)
- [BootstrapCreateCountryResponse](../../classes/src/review_ui/BootstrapCreateCountryResponse.md)
- [BootstrapGeneratePdfRequest](../../classes/src/review_ui/BootstrapGeneratePdfRequest.md)
- [ReviewGeneratePdfRequest](../../classes/src/review_ui/ReviewGeneratePdfRequest.md)
- [validate](../../functions/src/review_ui/ReviewGeneratePdfRequest/validate.md)
- [BootstrapGeneratePdfResponse](../../classes/src/review_ui/BootstrapGeneratePdfResponse.md)
- [BootstrapStampWorldRunRequest](../../classes/src/review_ui/BootstrapStampWorldRunRequest.md)
- [BootstrapStampWorldRunResponse](../../classes/src/review_ui/BootstrapStampWorldRunResponse.md)
- [BootstrapStampWorldStatusResponse](../../classes/src/review_ui/BootstrapStampWorldStatusResponse.md)
- [BootstrapAlbumEasyStatusResponse](../../classes/src/review_ui/BootstrapAlbumEasyStatusResponse.md)
- [StampWorldLogTailResponse](../../classes/src/review_ui/StampWorldLogTailResponse.md)
- [BootstrapAlbumEasySetPathRequest](../../classes/src/review_ui/BootstrapAlbumEasySetPathRequest.md)
- [BootstrapAlbumEasyPickPathRequest](../../classes/src/review_ui/BootstrapAlbumEasyPickPathRequest.md)
- [BootstrapSetAlbumsRootRequest](../../classes/src/review_ui/BootstrapSetAlbumsRootRequest.md)
- [BootstrapSetAlbumsRootResponse](../../classes/src/review_ui/BootstrapSetAlbumsRootResponse.md)
- [BootstrapPickAlbumsRootResponse](../../classes/src/review_ui/BootstrapPickAlbumsRootResponse.md)
- [BootstrapPickColnectFileRequest](../../classes/src/review_ui/BootstrapPickColnectFileRequest.md)
- [BootstrapPickColnectFileResponse](../../classes/src/review_ui/BootstrapPickColnectFileResponse.md)
- [BootstrapAlbumEasySetPathResponse](../../classes/src/review_ui/BootstrapAlbumEasySetPathResponse.md)
- [BootstrapAlbumEasyPickPathResponse](../../classes/src/review_ui/BootstrapAlbumEasyPickPathResponse.md)
- [StampWorldJobState](../../classes/src/review_ui/StampWorldJobState.md)
- [parse_extra_flags](../../functions/src/review_ui/parse_extra_flags.md)
- [preset_flags](../../functions/src/review_ui/preset_flags.md)
- [build_stampworld_args](../../functions/src/review_ui/build_stampworld_args.md)
- [quote_for_log](../../functions/src/review_ui/quote_for_log.md)
- [set_stampworld_state](../../functions/src/review_ui/set_stampworld_state.md)
- [stampworld_db_snapshot](../../functions/src/review_ui/stampworld_db_snapshot.md)
- [find_global_config_path](../../functions/src/review_ui/find_global_config_path.md)
- [save_runtime_string_setting](../../functions/src/review_ui/save_runtime_string_setting.md)
- [ensure_runtime_startup_files](../../functions/src/review_ui/ensure_runtime_startup_files.md)
- [is_directory_empty](../../functions/src/review_ui/is_directory_empty.md)
- [SiteAccountDetail](../../classes/src/review_ui/SiteAccountDetail.md)
- [AccountAuthStatusResponse](../../classes/src/review_ui/AccountAuthStatusResponse.md)
- [SaveSiteCredentialsRequest](../../classes/src/review_ui/SaveSiteCredentialsRequest.md)
- [SaveSiteCredentialsResponse](../../classes/src/review_ui/SaveSiteCredentialsResponse.md)
- [get_account_auth_status](../../functions/src/review_ui/get_account_auth_status.md)
- [account_auth_status_from_config](../../functions/src/review_ui/account_auth_status_from_config.md)
- [save_site_credentials](../../functions/src/review_ui/save_site_credentials.md)
- [save_site_credentials_to_path](../../functions/src/review_ui/save_site_credentials_to_path.md)
- [save_runtime_albumeasy_cli_path](../../functions/src/review_ui/save_runtime_albumeasy_cli_path.md)
- [save_runtime_bootstrap_albums_root_path](../../functions/src/review_ui/save_runtime_bootstrap_albums_root_path.md)
- [start_bootstrap_stampworld_job](../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [SelectedMatch](../../classes/src/review_ui/SelectedMatch.md)
- [FieldSelection](../../classes/src/review_ui/FieldSelection.md)
- [ApplyMatchesEnvelope](../../classes/src/review_ui/ApplyMatchesEnvelope.md)
- [ApplyMatchesResponse](../../classes/src/review_ui/ApplyMatchesResponse.md)
- [AddColnectStampRequest](../../classes/src/review_ui/AddColnectStampRequest.md)
- [AddColnectStampResponse](../../classes/src/review_ui/AddColnectStampResponse.md)
- [SaveLeftStampRequest](../../classes/src/review_ui/SaveLeftStampRequest.md)
- [SaveLeftStampResponse](../../classes/src/review_ui/SaveLeftStampResponse.md)
- [AddLeftStampRequest](../../classes/src/review_ui/AddLeftStampRequest.md)
- [AddLeftStampResponse](../../classes/src/review_ui/AddLeftStampResponse.md)
- [DeleteLeftStampRequest](../../classes/src/review_ui/DeleteLeftStampRequest.md)
- [DeleteLeftStampResponse](../../classes/src/review_ui/DeleteLeftStampResponse.md)
- [ClearLeftColnectLinkRequest](../../classes/src/review_ui/ClearLeftColnectLinkRequest.md)
- [ClearLeftColnectLinkResponse](../../classes/src/review_ui/ClearLeftColnectLinkResponse.md)
- [DeleteColnectCandidateRequest](../../classes/src/review_ui/DeleteColnectCandidateRequest.md)
- [DeleteColnectCandidateResponse](../../classes/src/review_ui/DeleteColnectCandidateResponse.md)
- [UndoColnectDeleteRequest](../../classes/src/review_ui/UndoColnectDeleteRequest.md)
- [UndoColnectDeleteResponse](../../classes/src/review_ui/UndoColnectDeleteResponse.md)
- [ShutdownRequest](../../classes/src/review_ui/ShutdownRequest.md)
- [ShutdownResponse](../../classes/src/review_ui/ShutdownResponse.md)
- [ColnectImportRequest](../../classes/src/review_ui/ColnectImportRequest.md)
- [ColnectImportResponse](../../classes/src/review_ui/ColnectImportResponse.md)
- [UpdateDefaultCatalogRequest](../../classes/src/review_ui/UpdateDefaultCatalogRequest.md)
- [UpdateDefaultCatalogResponse](../../classes/src/review_ui/UpdateDefaultCatalogResponse.md)
- [ConfigEditorField](../../classes/src/review_ui/ConfigEditorField.md)
- [ConfigEditorSection](../../classes/src/review_ui/ConfigEditorSection.md)
- [ConfigEditorDataResponse](../../classes/src/review_ui/ConfigEditorDataResponse.md)
- [SaveConfigEditorRequest](../../classes/src/review_ui/SaveConfigEditorRequest.md)
- [SaveConfigEditorResponse](../../classes/src/review_ui/SaveConfigEditorResponse.md)
- [ReviewUiSelected](../../classes/src/review_ui/ReviewUiSelected.md)
- [ReviewUiBootstrapOutcome](../../classes/src/review_ui/ReviewUiBootstrapOutcome.md)
- [ReviewUiRunOutcome](../../classes/src/review_ui/ReviewUiRunOutcome.md)
- [path_to_file_url](../../functions/src/review_ui/path_to_file_url.md)
- [open_local_path](../../functions/src/review_ui/open_local_path.md)
- [try_spawn_browser_command](../../functions/src/review_ui/try_spawn_browser_command.md)
- [try_spawn_browser_command_any](../../functions/src/review_ui/try_spawn_browser_command_any.md)
- [try_open_url_once](../../functions/src/review_ui/try_open_url_once.md)
- [open_review_ui_url](../../functions/src/review_ui/open_review_ui_url.md)
- [mime_from_path](../../functions/src/review_ui/mime_from_path.md)
- [bind_review_server](../../functions/src/review_ui/bind_review_server.md)
- [already_running_url](../../functions/src/review_ui/already_running_url.md)
- [PortProbeResult](../../classes/src/review_ui/PortProbeResult.md)
- [log_startup](../../functions/src/review_ui/log_startup.md)
- [probe_port](../../functions/src/review_ui/probe_port.md)
- [is_colnect_csv_file](../../functions/src/review_ui/is_colnect_csv_file.md)
- [count_colnect_csv_files_rec](../../functions/src/review_ui/count_colnect_csv_files_rec.md)
- [count_colnect_candidates_in_db](../../functions/src/review_ui/count_colnect_candidates_in_db.md)
- [scan_bootstrap_countries](../../functions/src/review_ui/scan_bootstrap_countries.md)
- [create_bootstrap_country_or_theme](../../functions/src/review_ui/create_bootstrap_country_or_theme.md)
- [current_bootstrap_albums_root](../../functions/src/review_ui/current_bootstrap_albums_root.md)
- [pick_bootstrap_albums_root_via_dialog](../../functions/src/review_ui/pick_bootstrap_albums_root_via_dialog.md)
- [pick_bootstrap_albums_root_via_dialog](../../functions/src/review_ui/pick_bootstrap_albums_root_via_dialog-2.md)
- [pick_and_copy_colnect_file_via_dialog](../../functions/src/review_ui/pick_and_copy_colnect_file_via_dialog.md)
- [pick_albumeasy_cli_path_via_dialog](../../functions/src/review_ui/pick_albumeasy_cli_path_via_dialog.md)
- [pick_albumeasy_cli_path_via_dialog](../../functions/src/review_ui/pick_albumeasy_cli_path_via_dialog-2.md)
- [pick_and_copy_colnect_file_for_db](../../functions/src/review_ui/pick_and_copy_colnect_file_for_db.md)
- [pick_and_copy_colnect_file_to_dir](../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)
- [set_bootstrap_albums_root_override](../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [resolve_bootstrap_albums_root](../../functions/src/review_ui/resolve_bootstrap_albums_root.md)
- [normalize_albums_root_candidate](../../functions/src/review_ui/normalize_albums_root_candidate.md)
- [resolve_image_abs](../../functions/src/review_ui/resolve_image_abs.md)
- [value_ref_to_string](../../functions/src/review_ui/value_ref_to_string.md)
- [load_stamp_all_fields](../../functions/src/review_ui/load_stamp_all_fields.md)
- [parse_query_param](../../functions/src/review_ui/parse_query_param.md)
- [parse_query_flag](../../functions/src/review_ui/parse_query_flag.md)
- [MatcherPayloadOptions](../../classes/src/review_ui/MatcherPayloadOptions.md)
- [build_matcher_payload](../../functions/src/review_ui/build_matcher_payload.md)
- [album_dir_for_ui](../../functions/src/review_ui/album_dir_for_ui.md)
- [open_folder_in_file_manager](../../functions/src/review_ui/open_folder_in_file_manager.md)
- [load_album_config](../../functions/src/review_ui/load_album_config.md)
- [safe_filename_token](../../functions/src/review_ui/safe_filename_token.md)
- [ext_from_data_url](../../functions/src/review_ui/ext_from_data_url.md)
- [store_uploaded_image_from_data_url](../../functions/src/review_ui/store_uploaded_image_from_data_url.md)
- [colnect_import_data](../../functions/src/review_ui/colnect_import_data.md)
- [run_review_ui_bootstrap](../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [run_review_ui](../../functions/src/review_ui/run_review_ui.md)
- [stampworld_pull_arguments_preserve_multiword_identity_and_snapshot_the_url](../../functions/src/review_ui/stampworld_pull_arguments_preserve_multiword_identity_and_snapshot_the_url.md)
- [site_accounts_preserve_saved_passwords_and_unrelated_config](../../functions/src/review_ui/site_accounts_preserve_saved_passwords_and_unrelated_config.md)
- [site_accounts_reject_invalid_config_without_overwriting_it](../../functions/src/review_ui/site_accounts_reject_invalid_config_without_overwriting_it.md)
- [sync_processed_colnect_candidates_marks_existing_stamp_links](../../functions/src/review_ui/sync_processed_colnect_candidates_marks_existing_stamp_links.md)
- [mark_colnect_candidate_processed_sets_flag_for_matching_candidate](../../functions/src/review_ui/mark_colnect_candidate_processed_sets_flag_for_matching_candidate.md)

# Imports

- `base64::Engine`
- `http_handlers::{serve_bootstrap_requests, serve_review_requests, ReviewHandlersStateRequests}`
- `log::debug`
- `rusqlite::{params, Connection}`
- `serde::{Deserialize, Serialize}`
- `std::collections::{HashMap, VecDeque}`
- `std::fs`
- `std::io::{BufRead, BufReader}`
- `std::path::{Path, PathBuf}`
- `std::process::{Command, Stdio}`
- `std::sync::atomic::{AtomicBool, Ordering}`
- `std::sync::{Mutex, OnceLock}`
- `std::thread`
- `std::time::Duration`
- `tiny_http::{Header, Method, Response, Server, StatusCode}`
- `views::{render_bootstrap_html, render_html}`
- `config_editor::{
    build_config_editor_data, build_config_editor_data_for_album, render_config_editor_assets,
    save_album_config_diff, save_full_config_values, update_default_catalog_code_for_album,
}`
- `matcher_service::{
    add_colnect_stamp_to_stamps, add_left_stamp, apply_matches, clear_left_colnect_link,
    delete_colnect_candidate, delete_left_stamp, load_left_stamps, load_right_candidates,
    save_left_stamp, sync_processed_colnect_candidates, undo_colnect_delete,
}`
- `std::io::Write`
- `std::io::{Read, Write}`
- `std::net::TcpStream`
- `super::matcher_service::{
        mark_colnect_candidate_processed, sync_processed_colnect_candidates,
    }`
- `rusqlite::Connection`
- `clap::Parser`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)