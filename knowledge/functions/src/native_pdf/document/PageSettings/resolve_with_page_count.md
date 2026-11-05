---
type: Rust Method
title: resolve_with_page_count
resource: src/native_pdf/document.rs#L236-L242
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_crop_marks/crop_marks_leave_all_measured_text_and_reservations_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/measured_footer_hints_parity_and_multiline_do_not_move_glyphs
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
  - target: functions/tests/native_pdf_header_date/date_hints_parity_and_multiline_reuse_measured_header_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/fixed_date_uses_explicit_formats_and_shared_header_geometry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_number/header_numbers_use_explicit_count_and_independent_style
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
---

# Signature

`pub fn resolve_with_page_count( &self, page_number: usize, total_pages: usize, ) -> Result<PageLayout>`

# Calls

- [resolve_with_context](../../../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)

# Called by

- [crop_marks_leave_all_measured_text_and_reservations_unchanged](../../../../../functions/tests/native_pdf_crop_marks/crop_marks_leave_all_measured_text_and_reservations_unchanged.md)
- [serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry](../../../../../functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry.md)
- [measured_footer_hints_parity_and_multiline_do_not_move_glyphs](../../../../../functions/tests/native_pdf_footer_date/measured_footer_hints_parity_and_multiline_do_not_move_glyphs.md)
- [fixed_footer_date_uses_formats_and_verified_bottom_geometry](../../../../../functions/tests/native_pdf_footer_date/fixed_footer_date_uses_formats_and_verified_bottom_geometry.md)
- [numbering_requires_known_total_and_uses_printed_start_not_parity](../../../../../functions/tests/native_pdf_footer_number/numbering_requires_known_total_and_uses_printed_start_not_parity.md)
- [signed_starts_tokens_and_hints_do_not_change_number_glyphs](../../../../../functions/tests/native_pdf_footer_number/signed_starts_tokens_and_hints_do_not_change_number_glyphs.md)
- [numbering_defaults_enablement_precedence_and_snapshots_are_explicit](../../../../../functions/tests/native_pdf_footer_number/numbering_defaults_enablement_precedence_and_snapshots_are_explicit.md)
- [coexistence_collisions_and_later_page_preflight_are_checked](../../../../../functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked.md)
- [changing_digit_width_and_font_metrics_are_validated_at_final_count](../../../../../functions/tests/native_pdf_footer_number/changing_digit_width_and_font_metrics_are_validated_at_final_count.md)
- [serialized_numbers_tokens_and_footer_coexistence_match_references](../../../../../functions/tests/native_pdf_footer_number/serialized_numbers_tokens_and_footer_coexistence_match_references.md)
- [serialized_dates_and_coexistence_match_frozen_reference_pdfs](../../../../../functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs.md)
- [date_hints_parity_and_multiline_reuse_measured_header_layout](../../../../../functions/tests/native_pdf_header_date/date_hints_parity_and_multiline_reuse_measured_header_layout.md)
- [fixed_date_uses_explicit_formats_and_shared_header_geometry](../../../../../functions/tests/native_pdf_header_date/fixed_date_uses_explicit_formats_and_shared_header_geometry.md)
- [header_numbers_use_explicit_count_and_independent_style](../../../../../functions/tests/native_pdf_header_number/header_numbers_use_explicit_count_and_independent_style.md)
- [defaults_missing_invalid_values_and_snapshot_are_explicit](../../../../../functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit.md)
- [signed_tokens_parity_multiline_and_shared_hints_are_measured](../../../../../functions/tests/native_pdf_header_number/signed_tokens_parity_multiline_and_shared_hints_are_measured.md)
- [late_width_collision_preflight_preserves_existing_output](../../../../../functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output.md)
- [wide_and_tall_titles_remain_below_both_header_elements](../../../../../functions/tests/native_pdf_header_number/wide_and_tall_titles_remain_below_both_header_elements.md)
- [serialized_header_numbers_match_reference_fonts_glyphs_and_geometry](../../../../../functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry.md)