---
type: Rust Method
title: set_border_colour
resource: src/native_pdf/document.rs#L198-L201
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_border_colour(&mut self, colour: AlbumBorderColour)`

# Called by

- [rendered_colours_match_reference_and_guide_stays_black](../../../../../functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [toml_defaults_and_independent_border_colours_are_explicit](../../../../../functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit.md)