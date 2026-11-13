---
type: Rust Function
title: err
resource: src/external_album.rs#L71-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/sql_integer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/scan_inventory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/copy_tree
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/list_albums
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render_pair
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/set_rendered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/verify_uploaded
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
  - target: functions/src/external_album/read_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/export_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/colour_validation_rejects_unsupported_forms_without_echoing_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/config_colour_is_applied_with_section_precedence_and_no_geometry_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/invalid_files_keys_and_geometry_return_actionable_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/missing_invalid_disabled_keys_redaction_and_unknown_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/strict_unsupported_invalid_and_redacted_definition_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/rendering_policy_has_no_missing_key_fallbacks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/proof_options_are_validated_without_output_or_secret_echoes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/missing_invalid_dates_and_fragments_report_footer_keys_without_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/footer_date_text_and_number_collisions_are_actionable_and_page_specific
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/all_font_variants_validate_exact_width_and_keyed_generation_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/invalid_missing_and_overflow_values_fail_with_redacted_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/invalid_missing_date_fields_return_redacted_keyed_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/date_collisions_with_text_and_numbers_identify_correction_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/overlapping_header_values_report_page_and_correction_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/missing_invalid_and_clipped_footer_values_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/missing_invalid_and_unusable_header_values_fail_with_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_precedence_snapshot_and_optional_extension_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/invalid_title_policy_fails_with_keyed_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn err(context: &str, error: impl std::fmt::Display) -> String`

# Called by

- [json](../../../functions/src/external_album/json.md)
- [sql_integer](../../../functions/src/external_album/sql_integer.md)
- [no_links](../../../functions/src/external_album/no_links.md)
- [digest](../../../functions/src/external_album/digest.md)
- [scan_inventory](../../../functions/src/external_album/scan_inventory.md)
- [inspect](../../../functions/src/external_album/inspect.md)
- [copy_tree](../../../functions/src/external_album/copy_tree.md)
- [schema](../../../functions/src/external_album/schema.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [paths](../../../functions/src/external_album/paths.md)
- [open](../../../functions/src/external_album/open.md)
- [details](../../../functions/src/external_album/details.md)
- [asset_directory](../../../functions/src/external_album/asset_directory.md)
- [list_albums](../../../functions/src/external_album/list_albums.md)
- [persist_pages](../../../functions/src/external_album/persist_pages.md)
- [import_album](../../../functions/src/external_album/import_album.md)
- [render_pair](../../../functions/src/external_album/render_pair.md)
- [verify_source](../../../functions/src/external_album/verify_source.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)
- [render](../../../functions/src/external_album/render.md)
- [verify_layout](../../../functions/src/external_album/verify_layout.md)
- [set_rendered](../../../functions/src/external_album/set_rendered.md)
- [verify_uploaded](../../../functions/src/external_album/verify_uploaded.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)
- [read_entry](../../../functions/src/external_album/read_entry.md)
- [export_layout](../../../functions/src/external_album/export_layout.md)
- [colour_validation_rejects_unsupported_forms_without_echoing_values](../../../functions/tests/native_pdf_border_colour/colour_validation_rejects_unsupported_forms_without_echoing_values.md)
- [config_colour_is_applied_with_section_precedence_and_no_geometry_changes](../../../functions/tests/native_pdf_border_colour/config_colour_is_applied_with_section_precedence_and_no_geometry_changes.md)
- [invalid_files_keys_and_geometry_return_actionable_redacted_errors](../../../functions/tests/native_pdf_config/invalid_files_keys_and_geometry_return_actionable_redacted_errors.md)
- [missing_invalid_disabled_keys_redaction_and_unknown_keys](../../../functions/tests/native_pdf_crop_marks/missing_invalid_disabled_keys_redaction_and_unknown_keys.md)
- [strict_unsupported_invalid_and_redacted_definition_errors](../../../functions/tests/native_pdf_decorative_border/strict_unsupported_invalid_and_redacted_definition_errors.md)
- [explicit_config_disabled_compatibility_and_templates](../../../functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates.md)
- [missing_corrupt_and_non_alpha_png_assets_fail_before_output](../../../functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output.md)
- [toml_defaults_and_independent_border_colours_are_explicit](../../../functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit.md)
- [rendering_policy_has_no_missing_key_fallbacks](../../../functions/tests/native_pdf_explicit_settings/rendering_policy_has_no_missing_key_fallbacks.md)
- [proof_options_are_validated_without_output_or_secret_echoes](../../../functions/tests/native_pdf_explicit_settings/proof_options_are_validated_without_output_or_secret_echoes.md)
- [missing_invalid_dates_and_fragments_report_footer_keys_without_values](../../../functions/tests/native_pdf_footer_date/missing_invalid_dates_and_fragments_report_footer_keys_without_values.md)
- [footer_date_text_and_number_collisions_are_actionable_and_page_specific](../../../functions/tests/native_pdf_footer_date/footer_date_text_and_number_collisions_are_actionable_and_page_specific.md)
- [late_footer_date_failure_preserves_existing_output_and_title_clearance](../../../functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance.md)
- [all_font_variants_validate_exact_width_and_keyed_generation_range](../../../functions/tests/native_pdf_footer_date/all_font_variants_validate_exact_width_and_keyed_generation_range.md)
- [invalid_missing_and_overflow_values_fail_with_redacted_keys](../../../functions/tests/native_pdf_footer_number/invalid_missing_and_overflow_values_fail_with_redacted_keys.md)
- [coexistence_collisions_and_later_page_preflight_are_checked](../../../functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked.md)
- [invalid_missing_date_fields_return_redacted_keyed_errors](../../../functions/tests/native_pdf_header_date/invalid_missing_date_fields_return_redacted_keyed_errors.md)
- [date_collisions_with_text_and_numbers_identify_correction_keys](../../../functions/tests/native_pdf_header_date/date_collisions_with_text_and_numbers_identify_correction_keys.md)
- [title_separation_and_invalid_later_page_preserve_output](../../../functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output.md)
- [overlapping_header_values_report_page_and_correction_keys](../../../functions/tests/native_pdf_header_number/overlapping_header_values_report_page_and_correction_keys.md)
- [defaults_missing_invalid_values_and_snapshot_are_explicit](../../../functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit.md)
- [late_width_collision_preflight_preserves_existing_output](../../../functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output.md)
- [missing_invalid_and_clipped_footer_values_fail_explicitly](../../../functions/tests/native_pdf_page_footer/missing_invalid_and_clipped_footer_values_fail_explicitly.md)
- [footer_fonts_fit_spill_and_no_partial_output](../../../functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output.md)
- [missing_invalid_and_unusable_header_values_fail_with_redacted_errors](../../../functions/tests/native_pdf_page_header/missing_invalid_and_unusable_header_values_fail_with_redacted_errors.md)
- [header_fit_spill_and_all_standard_fonts_reserve_without_clipping](../../../functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping.md)
- [title_precedence_snapshot_and_optional_extension_are_explicit](../../../functions/tests/native_pdf_page_title/title_precedence_snapshot_and_optional_extension_are_explicit.md)
- [invalid_title_policy_fails_with_keyed_redacted_errors](../../../functions/tests/native_pdf_page_title/invalid_title_policy_fails_with_keyed_redacted_errors.md)