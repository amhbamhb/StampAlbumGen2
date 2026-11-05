---
type: Rust Method
title: from_toml
resource: src/native_pdf/config.rs#L65-L67
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_border_colour/config_colour_is_applied_with_section_precedence_and_no_geometry_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/legacy_root_and_section_precedence_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/borders_are_required_and_even_margins_are_complete_or_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/invalid_files_keys_and_geometry_return_actionable_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/unapplied_settings_are_reported_by_key_not_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/user_config_controls_actual_rendered_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/templates_are_explicit_disabled_and_snapshot_is_independent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/invalid_layout_creates_no_pdf_and_preserves_existing_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/crop_marks_leave_all_measured_text_and_reservations_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output
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
  - target: functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/defaults_enablement_snapshots_and_colour_precedence_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/missing_invalid_dates_and_fragments_report_footer_keys_without_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/measured_footer_hints_parity_and_multiline_do_not_move_glyphs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/footer_date_text_and_number_collisions_are_actionable_and_page_specific
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/fixed_footer_date_uses_formats_and_verified_bottom_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/numbering_requires_known_total_and_uses_printed_start_not_parity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/signed_starts_tokens_and_hints_do_not_change_number_glyphs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/numbering_defaults_enablement_precedence_and_snapshots_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/invalid_missing_and_overflow_values_fail_with_redacted_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/changing_digit_width_and_font_metrics_are_validated_at_final_count
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
  - target: functions/tests/native_pdf_header_date/defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/invalid_missing_date_fields_return_redacted_keyed_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/date_hints_parity_and_multiline_reuse_measured_header_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/date_collisions_with_text_and_numbers_identify_correction_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/fixed_date_uses_explicit_formats_and_shared_header_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/header_numbers_use_explicit_count_and_independent_style
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/overlapping_header_values_report_page_and_correction_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/signed_tokens_parity_multiline_and_shared_hints_are_measured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/wide_and_tall_titles_remain_below_both_header_elements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_anchor_and_hint_share_layout_without_moving_glyphs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_multiline_padding_and_parity_are_independent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/missing_invalid_and_clipped_footer_values_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_defaults_colour_precedence_and_snapshots_preserve_number_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_and_header_hints_share_usable_content_and_title_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/header_uses_margin_coordinates_without_consuming_normal_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/multiline_padding_and_minimum_hint_share_resolved_measurements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/parity_alignment_tracks_physical_page_and_even_margins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/missing_invalid_and_unusable_header_values_fail_with_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/defaults_snapshots_and_unknown_header_keys_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_measures_once_and_reserves_font_dependent_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_fit_boundaries_and_hint_preserve_measured_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_precedence_snapshot_and_optional_extension_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/minimum_hint_only_expands_reservation_and_never_resizes_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/invalid_title_policy_fails_with_keyed_redacted_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_toml(source: &str) -> Result<Self>`

# Called by

- [config_colour_is_applied_with_section_precedence_and_no_geometry_changes](../../../../../functions/tests/native_pdf_border_colour/config_colour_is_applied_with_section_precedence_and_no_geometry_changes.md)
- [rendered_colours_match_reference_and_guide_stays_black](../../../../../functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black.md)
- [copied_template_values_reach_every_existing_command](../../../../../functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command.md)
- [legacy_root_and_section_precedence_are_explicit](../../../../../functions/tests/native_pdf_config/legacy_root_and_section_precedence_are_explicit.md)
- [borders_are_required_and_even_margins_are_complete_or_absent](../../../../../functions/tests/native_pdf_config/borders_are_required_and_even_margins_are_complete_or_absent.md)
- [invalid_files_keys_and_geometry_return_actionable_redacted_errors](../../../../../functions/tests/native_pdf_config/invalid_files_keys_and_geometry_return_actionable_redacted_errors.md)
- [unapplied_settings_are_reported_by_key_not_value](../../../../../functions/tests/native_pdf_config/unapplied_settings_are_reported_by_key_not_value.md)
- [user_config_controls_actual_rendered_paths](../../../../../functions/tests/native_pdf_config/user_config_controls_actual_rendered_paths.md)
- [config](../../../../../functions/tests/native_pdf_crop_marks/config.md)
- [templates_are_explicit_disabled_and_snapshot_is_independent](../../../../../functions/tests/native_pdf_crop_marks/templates_are_explicit_disabled_and_snapshot_is_independent.md)
- [invalid_layout_creates_no_pdf_and_preserves_existing_output](../../../../../functions/tests/native_pdf_crop_marks/invalid_layout_creates_no_pdf_and_preserves_existing_output.md)
- [crop_marks_leave_all_measured_text_and_reservations_unchanged](../../../../../functions/tests/native_pdf_crop_marks/crop_marks_leave_all_measured_text_and_reservations_unchanged.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [explicit_config_disabled_compatibility_and_templates](../../../../../functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates.md)
- [numeric_content_is_unchanged_and_invalid_geometry_preserves_output](../../../../../functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output.md)
- [toml_defaults_and_independent_border_colours_are_explicit](../../../../../functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit.md)
- [rendering_policy_has_no_missing_key_fallbacks](../../../../../functions/tests/native_pdf_explicit_settings/rendering_policy_has_no_missing_key_fallbacks.md)
- [proof_options_are_validated_without_output_or_secret_echoes](../../../../../functions/tests/native_pdf_explicit_settings/proof_options_are_validated_without_output_or_secret_echoes.md)
- [serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry](../../../../../functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../../../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [defaults_enablement_snapshots_and_colour_precedence_are_explicit](../../../../../functions/tests/native_pdf_footer_date/defaults_enablement_snapshots_and_colour_precedence_are_explicit.md)
- [missing_invalid_dates_and_fragments_report_footer_keys_without_values](../../../../../functions/tests/native_pdf_footer_date/missing_invalid_dates_and_fragments_report_footer_keys_without_values.md)
- [measured_footer_hints_parity_and_multiline_do_not_move_glyphs](../../../../../functions/tests/native_pdf_footer_date/measured_footer_hints_parity_and_multiline_do_not_move_glyphs.md)
- [footer_date_text_and_number_collisions_are_actionable_and_page_specific](../../../../../functions/tests/native_pdf_footer_date/footer_date_text_and_number_collisions_are_actionable_and_page_specific.md)
- [late_footer_date_failure_preserves_existing_output_and_title_clearance](../../../../../functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance.md)
- [fixed_footer_date_uses_formats_and_verified_bottom_geometry](../../../../../functions/tests/native_pdf_footer_date/fixed_footer_date_uses_formats_and_verified_bottom_geometry.md)
- [numbering_requires_known_total_and_uses_printed_start_not_parity](../../../../../functions/tests/native_pdf_footer_number/numbering_requires_known_total_and_uses_printed_start_not_parity.md)
- [signed_starts_tokens_and_hints_do_not_change_number_glyphs](../../../../../functions/tests/native_pdf_footer_number/signed_starts_tokens_and_hints_do_not_change_number_glyphs.md)
- [numbering_defaults_enablement_precedence_and_snapshots_are_explicit](../../../../../functions/tests/native_pdf_footer_number/numbering_defaults_enablement_precedence_and_snapshots_are_explicit.md)
- [invalid_missing_and_overflow_values_fail_with_redacted_keys](../../../../../functions/tests/native_pdf_footer_number/invalid_missing_and_overflow_values_fail_with_redacted_keys.md)
- [coexistence_collisions_and_later_page_preflight_are_checked](../../../../../functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked.md)
- [changing_digit_width_and_font_metrics_are_validated_at_final_count](../../../../../functions/tests/native_pdf_footer_number/changing_digit_width_and_font_metrics_are_validated_at_final_count.md)
- [serialized_numbers_tokens_and_footer_coexistence_match_references](../../../../../functions/tests/native_pdf_footer_number/serialized_numbers_tokens_and_footer_coexistence_match_references.md)
- [serialized_dates_and_coexistence_match_frozen_reference_pdfs](../../../../../functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs.md)
- [generation_date_requires_one_explicit_document_context](../../../../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)
- [defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit](../../../../../functions/tests/native_pdf_header_date/defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit.md)
- [invalid_missing_date_fields_return_redacted_keyed_errors](../../../../../functions/tests/native_pdf_header_date/invalid_missing_date_fields_return_redacted_keyed_errors.md)
- [date_hints_parity_and_multiline_reuse_measured_header_layout](../../../../../functions/tests/native_pdf_header_date/date_hints_parity_and_multiline_reuse_measured_header_layout.md)
- [date_collisions_with_text_and_numbers_identify_correction_keys](../../../../../functions/tests/native_pdf_header_date/date_collisions_with_text_and_numbers_identify_correction_keys.md)
- [title_separation_and_invalid_later_page_preserve_output](../../../../../functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output.md)
- [fixed_date_uses_explicit_formats_and_shared_header_geometry](../../../../../functions/tests/native_pdf_header_date/fixed_date_uses_explicit_formats_and_shared_header_geometry.md)
- [header_numbers_use_explicit_count_and_independent_style](../../../../../functions/tests/native_pdf_header_number/header_numbers_use_explicit_count_and_independent_style.md)
- [overlapping_header_values_report_page_and_correction_keys](../../../../../functions/tests/native_pdf_header_number/overlapping_header_values_report_page_and_correction_keys.md)
- [defaults_missing_invalid_values_and_snapshot_are_explicit](../../../../../functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit.md)
- [signed_tokens_parity_multiline_and_shared_hints_are_measured](../../../../../functions/tests/native_pdf_header_number/signed_tokens_parity_multiline_and_shared_hints_are_measured.md)
- [late_width_collision_preflight_preserves_existing_output](../../../../../functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output.md)
- [wide_and_tall_titles_remain_below_both_header_elements](../../../../../functions/tests/native_pdf_header_number/wide_and_tall_titles_remain_below_both_header_elements.md)
- [serialized_header_numbers_match_reference_fonts_glyphs_and_geometry](../../../../../functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry.md)
- [footer_anchor_and_hint_share_layout_without_moving_glyphs](../../../../../functions/tests/native_pdf_page_footer/footer_anchor_and_hint_share_layout_without_moving_glyphs.md)
- [footer_multiline_padding_and_parity_are_independent](../../../../../functions/tests/native_pdf_page_footer/footer_multiline_padding_and_parity_are_independent.md)
- [missing_invalid_and_clipped_footer_values_fail_explicitly](../../../../../functions/tests/native_pdf_page_footer/missing_invalid_and_clipped_footer_values_fail_explicitly.md)
- [footer_defaults_colour_precedence_and_snapshots_preserve_number_settings](../../../../../functions/tests/native_pdf_page_footer/footer_defaults_colour_precedence_and_snapshots_preserve_number_settings.md)
- [footer_fonts_fit_spill_and_no_partial_output](../../../../../functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output.md)
- [footer_and_header_hints_share_usable_content_and_title_fit](../../../../../functions/tests/native_pdf_page_footer/footer_and_header_hints_share_usable_content_and_title_fit.md)
- [serialized_footer_positions_colours_and_content_match_references](../../../../../functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references.md)
- [header_uses_margin_coordinates_without_consuming_normal_content](../../../../../functions/tests/native_pdf_page_header/header_uses_margin_coordinates_without_consuming_normal_content.md)
- [multiline_padding_and_minimum_hint_share_resolved_measurements](../../../../../functions/tests/native_pdf_page_header/multiline_padding_and_minimum_hint_share_resolved_measurements.md)
- [parity_alignment_tracks_physical_page_and_even_margins](../../../../../functions/tests/native_pdf_page_header/parity_alignment_tracks_physical_page_and_even_margins.md)
- [missing_invalid_and_unusable_header_values_fail_with_redacted_errors](../../../../../functions/tests/native_pdf_page_header/missing_invalid_and_unusable_header_values_fail_with_redacted_errors.md)
- [defaults_snapshots_and_unknown_header_keys_are_explicit](../../../../../functions/tests/native_pdf_page_header/defaults_snapshots_and_unknown_header_keys_are_explicit.md)
- [header_fit_spill_and_all_standard_fonts_reserve_without_clipping](../../../../../functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping.md)
- [title_measures_once_and_reserves_font_dependent_height](../../../../../functions/tests/native_pdf_page_title/title_measures_once_and_reserves_font_dependent_height.md)
- [title_fit_boundaries_and_hint_preserve_measured_text](../../../../../functions/tests/native_pdf_page_title/title_fit_boundaries_and_hint_preserve_measured_text.md)
- [title_precedence_snapshot_and_optional_extension_are_explicit](../../../../../functions/tests/native_pdf_page_title/title_precedence_snapshot_and_optional_extension_are_explicit.md)
- [minimum_hint_only_expands_reservation_and_never_resizes_text](../../../../../functions/tests/native_pdf_page_title/minimum_hint_only_expands_reservation_and_never_resizes_text.md)
- [invalid_title_policy_fails_with_keyed_redacted_errors](../../../../../functions/tests/native_pdf_page_title/invalid_title_policy_fails_with_keyed_redacted_errors.md)
- [title_pdf_positions_match_frozen_references_and_hint_controls_guide](../../../../../functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide.md)