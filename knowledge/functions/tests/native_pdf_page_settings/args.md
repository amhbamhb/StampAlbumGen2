---
type: Rust Function
title: args
resource: tests/native_pdf_page_settings.rs#L26-L39
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/dimensions/calculate_stamp_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/insert_test/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_fix_block_metadata/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_stamps/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/try_spawn_browser_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_folder_in_file_manager
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/windows_picker/picker_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/mark_desktop_file_trusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_first_run_installation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_cover_output_root/database_cli_reports_renderer_error_and_returns_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_integration_test/test_album_stamp_img_new_setting_affects_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_new_setting_usage/test_config_parameter_loading
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/tool_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/run_cli_with_args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_option_validation/test_ensure_binary_built
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/cli_param_matrix/cli_param_matrix_core_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/rasterize_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/user_config_controls_actual_rendered_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/independent_colours_reach_pdf_for_full_and_sparse_borders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/all_proof_policy_controls_actual_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/trace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/stampworld_pdf_cli/executable_routes_pdf_with_db_path_to_import_not_album_generation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn args(source: &str, name: &str) -> Result<Vec<f64>>`

# Called by

- [calculate_stamp_dimensions](../../../functions/src/album_utils/dimensions/calculate_stamp_dimensions.md)
- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [main](../../../functions/src/bin/backfill_image_dims/main.md)
- [main](../../../functions/src/bin/insert_test/main.md)
- [main](../../../functions/src/bin/inspect_image_paths/main.md)
- [main](../../../functions/src/bin/migrate_fix_block_metadata/main.md)
- [main](../../../functions/src/bin/migrate_stamps/main.md)
- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)
- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [try_spawn_browser_command](../../../functions/src/review_ui/try_spawn_browser_command.md)
- [open_folder_in_file_manager](../../../functions/src/review_ui/open_folder_in_file_manager.md)
- [picker_command](../../../functions/src/review_ui/windows_picker/picker_command.md)
- [mark_desktop_file_trusted](../../../functions/src/setup/mark_desktop_file_trusted.md)
- [ensure_first_run_installation](../../../functions/src/setup/ensure_first_run_installation.md)
- [database_cli_reports_renderer_error_and_returns_failure](../../../functions/tests/album_cover_output_root/database_cli_reports_renderer_error_and_returns_failure.md)
- [test_album_stamp_img_new_setting_affects_output](../../../functions/tests/album_stamp_img_integration_test/test_album_stamp_img_new_setting_affects_output.md)
- [test_config_parameter_loading](../../../functions/tests/album_stamp_img_new_setting_usage/test_config_parameter_loading.md)
- [tool_output](../../../functions/tests/albumeasy_baseline/tool_output.md)
- [run_cli_with_args](../../../functions/tests/cli_option_validation/run_cli_with_args.md)
- [test_ensure_binary_built](../../../functions/tests/cli_option_validation/test_ensure_binary_built.md)
- [cli_param_matrix_core_cases](../../../functions/tests/cli_param_matrix/cli_param_matrix_core_cases.md)
- [test_first_run_without_terminal_creates_defaults](../../../functions/tests/config_init_tests/test_first_run_without_terminal_creates_defaults.md)
- [rasterize_pdf](../../../functions/tests/external_album_backend/rasterize_pdf.md)
- [trace](../../../functions/tests/native_pdf_border_colour/trace.md)
- [user_config_controls_actual_rendered_paths](../../../functions/tests/native_pdf_config/user_config_controls_actual_rendered_paths.md)
- [trace](../../../functions/tests/native_pdf_crop_marks/trace.md)
- [actual_decorative_pixels_and_transparency_match](../../../functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match.md)
- [trace](../../../functions/tests/native_pdf_decorative_border/trace.md)
- [independent_colours_reach_pdf_for_full_and_sparse_borders](../../../functions/tests/native_pdf_explicit_settings/independent_colours_reach_pdf_for_full_and_sparse_borders.md)
- [all_proof_policy_controls_actual_output](../../../functions/tests/native_pdf_explicit_settings/all_proof_policy_controls_actual_output.md)
- [trace](../../../functions/tests/native_pdf_footer_date/trace.md)
- [trace](../../../functions/tests/native_pdf_footer_number/trace.md)
- [trace](../../../functions/tests/native_pdf_header_date/trace.md)
- [trace](../../../functions/tests/native_pdf_header_number/trace.md)
- [trace](../../../functions/tests/native_pdf_page_border/trace.md)
- [trace](../../../functions/tests/native_pdf_page_footer/trace.md)
- [trace](../../../functions/tests/native_pdf_page_header/trace.md)
- [settings](../../../functions/tests/native_pdf_page_settings/settings.md)
- [rendered_paths_match_margin_and_spacing_fixtures](../../../functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures.md)
- [title_pdf_positions_match_frozen_references_and_hint_controls_guide](../../../functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide.md)
- [executable_routes_pdf_with_db_path_to_import_not_album_generation](../../../functions/tests/stampworld_pdf_cli/executable_routes_pdf_with_db_path_to_import_not_album_generation.md)