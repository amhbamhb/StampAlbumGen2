---
type: Rust Method
title: set_even_margins
resource: src/native_pdf/document.rs#L129-L131
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_even_margins(&mut self, margins: EvenMargins)`

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [numeric_content_is_unchanged_and_invalid_geometry_preserves_output](../../../../../functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output.md)
- [late_footer_date_failure_preserves_existing_output_and_title_clearance](../../../../../functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance.md)
- [coexistence_collisions_and_later_page_preflight_are_checked](../../../../../functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked.md)
- [title_separation_and_invalid_later_page_preserve_output](../../../../../functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output.md)
- [footer_fonts_fit_spill_and_no_partial_output](../../../../../functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output.md)
- [header_fit_spill_and_all_standard_fonts_reserve_without_clipping](../../../../../functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping.md)
- [settings](../../../../../functions/tests/native_pdf_page_settings/settings.md)
- [invalid_commands_geometry_and_output_fail_explicitly](../../../../../functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly.md)