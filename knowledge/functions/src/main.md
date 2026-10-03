---
type: Rust Function
title: main
resource: src/main.rs#L39-L617
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/write_startup_probe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/write_startup_probe_snapshot
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/setup_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/uninstall_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/install_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/align_process_working_dir_to_runtime_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/check_config_staleness
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_windows_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/set_pdf_dx_logging_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/validate_os
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui_bootstrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_inbox_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_default_sort_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_odd_row_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_even_row_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_first_column_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_second_column_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_series_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_description_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_watermark_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_design_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_engraving_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_perforation_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_context_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_colnect_links_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_colnect_image_url_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_colnect_metadata_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_replace_sg_number_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_right_unmatched_only_default_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/albumeasy_auto_pdf_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/review_ui_catalog_family_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/create_config_from_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/validation/validate_user_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/validation/validate_year_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/validation/print_validation_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/validation/validate_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn main() -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [write_startup_probe](../../functions/src/setup/write_startup_probe.md)
- [log_runtime_event](../../functions/src/review_ui/log_runtime_event.md)
- [write_startup_probe_snapshot](../../functions/src/setup/write_startup_probe_snapshot.md)
- [setup_logging](../../functions/src/setup/setup_logging.md)
- [uninstall_linux_desktop_integration](../../functions/src/setup/uninstall_linux_desktop_integration.md)
- [install_linux_desktop_integration](../../functions/src/setup/install_linux_desktop_integration.md)
- [ensure_config_and_template_files](../../functions/src/config/ensure_config_and_template_files.md)
- [align_process_working_dir_to_runtime_layout](../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)
- [with_defaults](../../functions/src/config/Config/with_defaults.md)
- [check_config_staleness](../../functions/src/config/check_config_staleness.md)
- [ensure_linux_desktop_integration](../../functions/src/setup/ensure_linux_desktop_integration.md)
- [ensure_windows_desktop_integration](../../functions/src/setup/ensure_windows_desktop_integration.md)
- [set_pdf_dx_logging_enabled](../../functions/src/album_globals/set_pdf_dx_logging_enabled.md)
- [validate_os](../../functions/src/setup/validate_os.md)
- [run_review_ui_bootstrap](../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [effective_colnect_inbox_path](../../functions/src/config/Config/effective_colnect_inbox_path.md)
- [load_config_with_override](../../functions/src/config/load_config_with_override.md)
- [run_review_ui](../../functions/src/review_ui/run_review_ui.md)
- [effective_default_catalog_code](../../functions/src/config/Config/effective_default_catalog_code.md)
- [review_ui_default_sort_mode](../../functions/src/config/Config/review_ui_default_sort_mode.md)
- [review_ui_odd_row_color](../../functions/src/config/Config/review_ui_odd_row_color.md)
- [review_ui_even_row_color](../../functions/src/config/Config/review_ui_even_row_color.md)
- [review_ui_first_column_color](../../functions/src/config/Config/review_ui_first_column_color.md)
- [review_ui_second_column_color](../../functions/src/config/Config/review_ui_second_column_color.md)
- [review_ui_series_default_checked](../../functions/src/config/Config/review_ui_series_default_checked.md)
- [review_ui_description_default_checked](../../functions/src/config/Config/review_ui_description_default_checked.md)
- [review_ui_watermark_default_checked](../../functions/src/config/Config/review_ui_watermark_default_checked.md)
- [review_ui_design_default_checked](../../functions/src/config/Config/review_ui_design_default_checked.md)
- [review_ui_engraving_default_checked](../../functions/src/config/Config/review_ui_engraving_default_checked.md)
- [review_ui_perforation_default_checked](../../functions/src/config/Config/review_ui_perforation_default_checked.md)
- [review_ui_context_default_checked](../../functions/src/config/Config/review_ui_context_default_checked.md)
- [review_ui_colnect_links_default_checked](../../functions/src/config/Config/review_ui_colnect_links_default_checked.md)
- [review_ui_colnect_image_url_default_checked](../../functions/src/config/Config/review_ui_colnect_image_url_default_checked.md)
- [review_ui_colnect_metadata_default_checked](../../functions/src/config/Config/review_ui_colnect_metadata_default_checked.md)
- [review_ui_replace_sg_number_default_checked](../../functions/src/config/Config/review_ui_replace_sg_number_default_checked.md)
- [review_ui_right_unmatched_only_default_checked](../../functions/src/config/Config/review_ui_right_unmatched_only_default_checked.md)
- [albumeasy_auto_pdf_enabled](../../functions/src/config/Config/albumeasy_auto_pdf_enabled.md)
- [effective_albumeasy_cli_path](../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [review_ui_catalog_family_options](../../functions/src/config/Config/review_ui_catalog_family_options.md)
- [create_config_from_template](../../functions/src/config/create_config_from_template.md)
- [validate_user_input](../../functions/src/validation/validate_user_input.md)
- [validate_year_range](../../functions/src/validation/validate_year_range.md)
- [print_validation_error](../../functions/src/validation/print_validation_error.md)
- [validate_source](../../functions/src/validation/validate_source.md)
- [run_database_mode](../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../functions/src/modes/run_scraping_mode.md)